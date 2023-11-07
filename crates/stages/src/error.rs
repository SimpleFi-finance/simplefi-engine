use std::{error, sync::mpsc::SendError};
use thiserror::Error;


/// A stage execution error.
#[derive(Error, Debug)]
pub enum StageError {
    /// The stage encountered an error related to a block.
    #[error("Stage encountered a block error in block {number}", number = block)]
    Block {
        /// The block that caused the error.
        block: u64,
    },
    // /// The stage encountered a database error.
    // #[error("An internal database error occurred: {0}")]
    // Database(#[from] DatabaseError),
    /// Invalid checkpoint passed to the stage
    #[error("Invalid stage checkpoint: {0}")]
    StageCheckpoint(u64),
    /// Download channel closed
    #[error("Download channel closed")]
    ChannelClosed,
    // /// The stage encountered a database integrity error.
    // // #[error("A database integrity error occurred: {0}")]
    // // DatabaseIntegrity(#[from] ProviderError),
    // /// Invalid download response. Applicable for stages which
    // /// rely on external downloaders
    // #[error("Invalid download response: {0}")]
    // Download(#[from] DownloadError),
    // /// Internal error
    // #[error(transparent)]
    // Internal(#[from] RethError),
    /// The stage encountered a recoverable error.
    ///
    /// These types of errors are caught by the [Pipeline][crate::Pipeline] and trigger a restart
    /// of the stage.
    #[error(transparent)]
    Recoverable(Box<dyn std::error::Error + Send + Sync>),
    /// The stage encountered a fatal error.
    ///
    /// These types of errors stop the pipeline.
    #[error(transparent)]
    Fatal(Box<dyn std::error::Error + Send + Sync>),
}

// TODO: update
impl StageError {
    /// If the error is fatal the pipeline will stop.
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
                StageError::StageCheckpoint(_) |
                StageError::ChannelClosed |
                StageError::Fatal(_)
        )
    }
}

/// A pipeline execution error.
#[derive(Error, Debug)]
pub enum PipelineError {
    /// The pipeline encountered an irrecoverable error in one of the stages.
    #[error("A stage encountered an irrecoverable error.")]
    Stage(#[from] StageError),
    /// The pipeline encountered a database error.
    // #[error("A database error occurred.")]
    // Database(#[from] DbError),
    /// The pipeline encountered an irrecoverable error in one of the stages.
    // #[error("An interface error occurred.")]
    // Interface(#[from] RethError),
    /// The pipeline encountered an error while trying to send an event.
    #[error("The pipeline encountered an error while trying to send an event.")]
    Channel(#[from] SendError<String>),
    /// The stage encountered an internal error.
    #[error(transparent)]
    Internal(Box<dyn std::error::Error + Send + Sync>),
}
