use crate::{
    BlockHashOrNumber, BlockNumber,
};

/// Bundled errors variants thrown by various providers.
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum RpcProviderError {
    /// The header number was not found.
    #[error("block number {0:?} does not found in rpc provider")]
    BlockNotFound(BlockNumber),

    /// A block txs not found.
    #[error("block txs not found for block #{0}")]
    BlockTransactions(BlockNumber),

    /// block traces not found
    #[error("block traces not found for block #{0}")]
    BlockTraces(BlockNumber),

    /// block logs not found
    #[error("block logs not found for block #{0}")]
    BlockLogs(BlockNumber),

    /// when required header related data was not found but was required.
    #[error("no header found for {0:?}")]
    HeaderNotFound(BlockHashOrNumber),

    /// Range queried too large.
    #[error("Range queried is too large, reduce range")]
    RangeTooLarge,

    /// Socket error
    #[error("Socket error: {0}")]
    SocketError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
