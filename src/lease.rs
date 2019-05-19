use crate::errors::Error;
use crate::v3::rpc::{
    LeaseGrantRequest, LeaseGrantResponse, LeaseKeepAliveRequest, LeaseKeepAliveResponse,
    LeaseRevokeRequest, LeaseRevokeResponse,
};
use crate::v3::rpc_grpc::{Lease as LeaseTrait, LeaseClient};
use futures::Future;
use grpc::ClientStub;
use grpc::RequestOptions;
use std::sync::Arc;

pub struct Lease {
    pub client: Arc<grpc::Client>,
    pub lease_id: i64,
    lease_client: LeaseClient,
    pub ttl: i64,
}

impl Lease {
    pub fn new(ttl: i64, client: Arc<grpc::Client>) -> impl Future<Item = Lease, Error = Error> {
        trace!("Initializing lease...");

        let lease_client = LeaseClient::with_client(client.clone());
        Self::get_lease(0, ttl, lease_client).and_then(move |response| {
            if response.error.len() > 0 {
                Err(Error::Unrecoverable(response.error.clone()))
            } else {
                Ok(Lease {
                    client: client.clone(),
                    lease_id: response.ID,
                    lease_client: LeaseClient::with_client(client),
                    ttl: response.TTL,
                })
            }
        })
    }

    fn get_lease(
        lease_id: i64,
        ttl: i64,
        lease_client: LeaseClient,
    ) -> impl Future<Item = LeaseGrantResponse, Error = Error> {
        let request_options = RequestOptions::new();
        let mut lease_grant_request = LeaseGrantRequest::new();
        lease_grant_request.ID = lease_id;
        lease_grant_request.TTL = ttl;
        lease_client
            .lease_grant(request_options, lease_grant_request)
            .drop_metadata()
            .from_err()
    }

    fn revoke_lease(
        lease_id: i64,
        lease_client: &LeaseClient,
    ) -> impl Future<Item = LeaseRevokeResponse, Error = Error> {
        let request_options = RequestOptions::new();
        let mut lease_revoke_request = LeaseRevokeRequest::new();
        lease_revoke_request.ID = lease_id;
        lease_client
            .lease_revoke(request_options, lease_revoke_request)
            .drop_metadata()
            .from_err()
    }
}

impl Lease {
    pub fn keep_alive<S>(
        &self,
        keep_alive_requests: S,
    ) -> grpc::StreamingResponse<LeaseKeepAliveResponse>
    where
        S: futures::Stream<Item = LeaseKeepAliveRequest, Error = grpc::Error> + Send + 'static,
    {
        self.lease_client.lease_keep_alive(
            RequestOptions::new(),
            grpc::StreamingRequest::new(keep_alive_requests),
        )
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        debug!("Dropping lease with id: {}", self.lease_id);

        match Self::revoke_lease(self.lease_id, &self.lease_client).wait() {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Could not revoke lease: {}, with ttl: {}. {}",
                    self.lease_id, self.ttl, e
                );
            }
        }
    }
}
