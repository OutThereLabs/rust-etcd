use std::convert::From;

use grpc;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "GRPC error: {}", _0)]
    Grpc(grpc::Error),
    #[fail(display = "Unrecoverable error: {}", _0)]
    Unrecoverable(String),
}

impl From<grpc::Error> for Error {
    fn from(rpc_error: grpc::Error) -> Error {
        Error::Grpc(rpc_error)
    }
}
