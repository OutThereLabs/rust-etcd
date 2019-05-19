use crate::v3::rpc::{LeaseKeepAliveRequest, LeaseKeepAliveResponse};
use crate::Lease;
use actix::prelude::*;
use futures::{future, sync::mpsc, Stream};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_SESSION_TTL: i64 = 60;

#[derive(Clone, Debug, Message)]
pub enum SessionManagerState {
    Disconnected,
    Connecting,
    Connected { lease_id: i64 },
}

pub struct SessionManager {
    client: Arc<grpc::Client>,
    ttl: i64,
    lease: Option<Lease>,
    state: SessionManagerState,
    state_subscribers: HashSet<actix::Recipient<SessionManagerState>>,
    health_checks: Option<actix::SpawnHandle>,
}

impl SessionManager {
    pub fn new(client: Arc<grpc::Client>, ttl: Option<i64>) -> Self {
        SessionManager {
            client,
            ttl: ttl.unwrap_or(DEFAULT_SESSION_TTL),
            lease: None,
            state: SessionManagerState::Disconnected,
            state_subscribers: Default::default(),
            health_checks: None,
        }
    }

    fn set_state(&mut self, state: SessionManagerState) {
        debug!(
            "Session manager transitioning state {:?} -> {:?}",
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

impl Actor for SessionManager {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Start the session state machine
        ctx.address().do_send(EstablishSession);
    }
}

#[derive(Message)]
struct EstablishSession;

impl Handler<EstablishSession> for SessionManager {
    type Result = ();

    fn handle(&mut self, _msg: EstablishSession, ctx: &mut Self::Context) {
        match self.state {
            SessionManagerState::Disconnected => {
                self.set_state(SessionManagerState::Connecting);

                ctx.spawn(
                    actix::fut::wrap_future::<_, Self>(Lease::new(self.ttl, self.client.clone()))
                        .map_err(|err, _act, ctx| {
                            error!("Error obtaining lease: {}", err);
                            ctx.run_later(Duration::from_secs(3), |_, ctx| ctx.terminate());
                        })
                        .and_then(|lease, _act, ctx| {
                            let lease_id = lease.lease_id;
                            let (tx, rx) = mpsc::unbounded();
                            let requests =
                                ctx.run_interval(Duration::from_secs(1), move |_act, ctx| {
                                    let mut request = LeaseKeepAliveRequest::new();
                                    request.set_ID(lease_id);
                                    debug!("Sending lease keep alive request: {:?}", request);
                                    let _ = tx.unbounded_send(request).map_err(|err| {
                                        error!("Error sending health check: {:?}", err);
                                        ctx.run_later(Duration::from_secs(3), |_, ctx| {
                                            ctx.terminate()
                                        });
                                    });
                                });
                            let responses = lease.keep_alive(
                                rx.map_err(|_| grpc::Error::Other("keep alive channel error")),
                            );
                            actix::fut::wrap_future::<_, Self>(future::result(
                                ctx.address().try_send(SessionEstablished {
                                    lease,
                                    health_checks: requests,
                                    health_check_responses: responses.drop_metadata(),
                                }),
                            ))
                            .map_err(|err, _act, ctx| {
                                warn!("Session manager error: {:?}", err);
                                ctx.run_later(Duration::from_secs(3), |_, ctx| ctx.terminate());
                            })
                        }),
                );
            }
            _ => warn!(
                "Attempting to establish session while in state: {:?}",
                self.state
            ),
        }
    }
}

#[derive(Message)]
pub struct SubscribeSessionState(pub Recipient<SessionManagerState>);

impl Handler<SubscribeSessionState> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: SubscribeSessionState, _ctx: &mut Self::Context) {
        self.state_subscribers.insert(msg.0);
    }
}

#[derive(Message)]
struct SessionEstablished<S: Stream + 'static> {
    lease: Lease,
    health_checks: actix::SpawnHandle,
    health_check_responses: S,
}

impl<S: Stream<Item = LeaseKeepAliveResponse, Error = grpc::Error> + 'static>
    Handler<SessionEstablished<S>> for SessionManager
{
    type Result = ();

    fn handle(&mut self, msg: SessionEstablished<S>, ctx: &mut Self::Context) {
        debug!(
            "Session established with lease: {:?} and ttl: {:?}",
            msg.lease.lease_id, self.ttl
        );
        let lease_id = msg.lease.lease_id;
        self.lease = Some(msg.lease);
        self.health_checks = Some(msg.health_checks);
        Self::add_stream(msg.health_check_responses, ctx);

        self.set_state(SessionManagerState::Connected { lease_id });
    }
}

impl StreamHandler<LeaseKeepAliveResponse, grpc::Error> for SessionManager {
    fn handle(&mut self, msg: LeaseKeepAliveResponse, ctx: &mut Self::Context) {
        // Consuming and ignoring keep alive responses
        debug!(
            "Received lease keep alive response: {:?}, ttl: {:?}",
            msg, msg.TTL
        );
        if msg.TTL == 0 {
            warn!("The lease for this session is no longer valid. Terminating.");
            ctx.terminate()
        }
    }

    fn error(&mut self, err: grpc::Error, _ctx: &mut Self::Context) -> Running {
        warn!("Error health check response: {:?}", err);
        Running::Stop
    }

    /// Method is called when stream finishes.
    ///
    /// By default this method stops actor execution.
    fn finished(&mut self, ctx: &mut Self::Context) {
        warn!("Lease keep alive stream finished. Terminating.");
        ctx.terminate();
    }
}

impl actix::Supervised for SessionManager {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        warn!("Session manager is restarting");
        if let Some(handler) = self.health_checks.take() {
            ctx.cancel_future(handler);
        };
        self.lease = None;
        self.set_state(SessionManagerState::Disconnected);
    }
}
