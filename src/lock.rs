use crate::errors::Error;
use crate::lease::Lease;
use crate::v3::v3lock::{
    LockRequest as GrpcLockRequest, LockResponse, UnlockRequest, UnlockResponse,
};
use crate::v3::v3lock_grpc::{Lock as LockTrait, LockClient};
use futures::Future;
use grpc::ClientStub;
use grpc::RequestOptions;
use std::rc::Rc;

pub struct Lock {
    pub key: Vec<u8>,
    lease: Rc<Lease>,
}

impl Lock {
    pub fn new(name: Vec<u8>, lease: Rc<Lease>) -> impl Future<Item = Lock, Error = Error> {
        let lock_client = LockClient::with_client(lease.client.clone());
        Lock::get_lock(name, lock_client).and_then(move |response| {
            Ok(Lock {
                key: response.key,
                lease: lease.clone(),
            })
        })
    }

    fn unlock(
        key: Vec<u8>,
        lock_client: LockClient,
    ) -> impl Future<Item = UnlockResponse, Error = Error> {
        let request_options = RequestOptions::new();
        let mut unlock_request = UnlockRequest::new();
        unlock_request.key = key;
        lock_client
            .unlock(request_options, unlock_request)
            .drop_metadata()
            .from_err()
    }

    fn get_lock(
        name: Vec<u8>,
        lock_client: LockClient,
    ) -> impl Future<Item = LockResponse, Error = Error> {
        let request_options = RequestOptions::new();
        let mut lock_request = GrpcLockRequest::new();
        lock_request.name = name;
        lock_client
            .lock(request_options, lock_request)
            .drop_metadata()
            .from_err()
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        trace!("Revoking lock");

        let lock_client = LockClient::with_client(self.lease.client.clone());
        let _ = Lock::unlock(self.key.clone(), lock_client).map_err(|error| {
            error!("Could not revoke lock: {}", error);

            error
        });
    }
}
