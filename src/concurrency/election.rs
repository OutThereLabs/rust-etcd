use crate::v3::v3election::{CampaignRequest, LeaderKey};
use crate::v3::v3election_grpc::{Election, ElectionClient};
use crate::{SessionManager, SessionManagerState, SubscribeSessionState};
use actix::prelude::*;
use futures::future;
use grpc::ClientStub;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashSet;

#[derive(Clone, Debug, Message)]
pub enum ElectionState {
    Unelected,
    Campaigning,
    Elected,
}

pub struct ElectionCandidate {
    client: Arc<grpc::Client>,
    election_name: Vec<u8>,
    candidate_id: Vec<u8>,
    leader: Option<LeaderKey>,
    lease_id: Option<i64>,
    session_manager: Addr<SessionManager>,
    state: ElectionState,
    state_subscribers: HashSet<Recipient<ElectionState>>,
}

impl ElectionCandidate {
    pub fn new<T: Into<Vec<u8>>>(
        client: Arc<grpc::Client>,
        election_mame: T,
        candidate_id: T,
        session_manager: Addr<SessionManager>,
    ) -> Self {
        ElectionCandidate {
            client,
            election_name: election_mame.into(),
            candidate_id: candidate_id.into(),
            leader: None,
            lease_id: None,
            session_manager,
            state: ElectionState::Unelected,
            state_subscribers: Default::default(),
        }
    }

    fn set_state(&mut self, state: ElectionState) {
        debug!(
            "Election candidate transitioning state {:?} -> {:?}",
            self.state, state
        );
        self.state = state;
        for subscriber in self.state_subscribers.iter() {
            let _ = subscriber.try_send(self.state.clone()).map_err(|err| {
                warn!("Cannot update subscriber state: {}", err);
            });
        }
    }
}

impl Actor for ElectionCandidate {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let recipient = ctx.address().recipient();
        self.session_manager
            .do_send(SubscribeSessionState(recipient))
    }
}

#[derive(Message)]
pub struct ElectionStateSubscribe(pub Recipient<ElectionState>);

impl Handler<ElectionStateSubscribe> for ElectionCandidate {
    type Result = ();

    fn handle(&mut self, msg: ElectionStateSubscribe, _ctx: &mut Self::Context) {
        self.state_subscribers.insert(msg.0);
    }
}

#[derive(Message)]
struct StartCampaigning;

impl Handler<StartCampaigning> for ElectionCandidate {
    type Result = ();

    fn handle(&mut self, _msg: StartCampaigning, ctx: &mut Self::Context) {
        match self.state {
            ElectionState::Unelected => {
                self.set_state(ElectionState::Campaigning);

                let mut campaign_request = CampaignRequest::new();
                campaign_request.set_name(self.election_name.clone());
                campaign_request.set_value(self.candidate_id.clone());
                campaign_request.set_lease(self.lease_id.expect("Election should have lease"));

                ctx.spawn(
                    actix::fut::wrap_future::<_, Self>(
                        ElectionClient::with_client(self.client.clone())
                            .campaign(grpc::RequestOptions::new(), campaign_request)
                            .drop_metadata(),
                    )
                    .map_err(|err, _act, _ctx| error!("Campaign error: {}", err))
                    .and_then(move |mut campaign_response, _act, ctx| {
                        actix::fut::wrap_future::<_, Self>(future::result(ctx.address().try_send(
                            Elected {
                                leader: campaign_response.leader.take(),
                            },
                        )))
                        .map_err(|err, _act, ctx| {
                            warn!("Election candidate error: {:?}", err);
                            ctx.run_later(Duration::from_secs(3), |_, ctx| ctx.terminate());
                        })
                    }),
                );
            }
            _ => warn!("Attempting to campaign while in state: {:?}", self.state),
        }
    }
}

#[derive(Message)]
struct Elected {
    leader: Option<LeaderKey>,
}

impl Handler<Elected> for ElectionCandidate {
    type Result = ();

    fn handle(&mut self, msg: Elected, _ctx: &mut Self::Context) {
        self.leader = msg.leader;
        self.set_state(ElectionState::Elected);
    }
}

impl Handler<SessionManagerState> for ElectionCandidate {
    type Result = ();

    fn handle(&mut self, msg: SessionManagerState, ctx: &mut Self::Context) {
        match msg {
            SessionManagerState::Connecting => {}
            SessionManagerState::Disconnected => match self.state {
                ElectionState::Unelected => {}
                ElectionState::Elected | ElectionState::Campaigning => {
                    warn!("Session disconnected. Terminating to run election with new lease.");
                    ctx.terminate();
                }
            },
            SessionManagerState::Connected { lease_id } => {
                debug!("Session established event received, starting to campaign!");
                self.lease_id = Some(lease_id);
                ctx.address()
                    .try_send(StartCampaigning)
                    .expect("Candidate mailbox issue");
            }
        }
    }
}

impl actix::Supervised for ElectionCandidate {
    fn restarting(&mut self, _ctx: &mut Self::Context) {
        warn!("Election candidate is restarting");
        self.leader = None;
        self.lease_id = None;
        self.set_state(ElectionState::Unelected);
    }
}
