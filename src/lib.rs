#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate grpc;

#[cfg(feature = "actix")]
mod concurrency;
mod errors;
mod lease;
mod lock;
pub mod v3;

#[cfg(feature = "actix")]
pub use self::concurrency::election::{ElectionCandidate, ElectionState, ElectionStateSubscribe};
#[cfg(feature = "actix")]
pub use self::concurrency::session::{SessionManager, SessionManagerState, SubscribeSessionState};
pub use self::errors::Error;
pub use self::lease::Lease;
pub use self::lock::Lock;
