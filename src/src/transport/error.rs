use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Failed to bind socket. Error: {0:?}")]
    BindFailed(#[from] std::io::Error),

    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error("Netcode error: {0:?}")]
    NetcodeError(#[from] renet_netcode::NetcodeError),

    #[error("Netcode Transport error: {0:?}")]
    NetcodeTransportError(#[from] renet_netcode::NetcodeTransportError),
}