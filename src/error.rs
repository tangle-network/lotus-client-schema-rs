use thiserror::Error;

#[derive(Error, Debug)]
pub enum LotusError {
    #[error("Client error: {0}")]
    ClientError(#[from] jsonrpsee_core::ClientError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("JSON-RPC error: {0}")]
    JsonRpcError(#[from] jsonrpc_core::Error),
}
