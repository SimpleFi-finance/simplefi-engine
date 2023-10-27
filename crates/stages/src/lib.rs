mod pipeline;

mod error;
pub use error::PipelineError;
mod stages;

mod stage;
pub use stage::{Stage, StageId};

pub mod util;