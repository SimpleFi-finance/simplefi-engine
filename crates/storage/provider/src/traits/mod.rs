use db::table::Table;
use interfaces::Result;

mod header;
pub use header::{HeaderProvider, HeaderWriter};

mod block;
pub use block::{BlockReader, BlockSource, BlockWriter};

mod block_id;
pub use block_id::{BlockNumReader, BlockNumWriter};

mod block_hash;
pub use block_hash::{BlockHashReader, BlockHashWriter};

mod transactions;
pub use transactions::{TransactionsProvider, TransactionsWriter};

mod logs;
pub use logs::{LogsProvider, LogsWriter, StoredOrDecodedLog};

mod block_body_index;
pub use block_body_index::{BlockBodyIndicesProvider, BlockBodyIndicesWriter};

mod abis;
pub use abis::{AbiProvider, AbiWriter};

mod tracking;
pub(crate) mod volumetric;
pub use volumetric::*;

pub mod markets;
pub mod protocols;

pub use markets::*;
pub use protocols::*;
pub use tracking::{TrackingProvider, TrackingWriter};

mod stage_checkpoints;
pub use stage_checkpoints::{StageCheckpointProvider, StageCheckpointWriter};

pub trait ShardedTableProvider: Send + Sync {
    fn get_latest_shard<T: Table>(&self, prefix:  &[u8]) -> Result<Option<&[u8]>>;

    fn get_shard<T: Table>(&self, key:  T::Key) -> Result<Option<T::Value>>;
}