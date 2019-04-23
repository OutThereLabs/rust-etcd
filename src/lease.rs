use crate::errors::Error;
use crate::rpc::{LeaseGrantRequest, LeaseGrantResponse, LeaseRevokeRequest, LeaseRevokeResponse};
use crate::rpc_grpc::{Lease as LeaseTrait, LeaseClient};
use futures::Future;
use grpc::ClientStub;
use grpc::RequestOptions;
use std::sync::Arc;

pub struct Lease {
    pub client: Arc<grpc::Client>,
    pub lease_id: i64,
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
        lease_client: LeaseClient,
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

impl Drop for Lease {
    fn drop(&mut self) {
        trace!("Dropping lease...");

        let lease_client = LeaseClient::with_client(self.client.clone());
        let _ = Self::revoke_lease(self.lease_id, lease_client).map_err(|error| {
            error!("Could not revoke lease: {}", error);

            error
        });
    }
}
