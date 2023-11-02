use simp_primitives::{LogNumber, TxNumber, BlockNumber};
use sip_codecs::{main_codec, Compact};

/// The storage of the block body indices
///
/// It has the pointer to the transaction Number of the first
#[main_codec]
/// transaction in the block and the total number of transactions
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct TxLogs {
    pub log_ids: Vec<TxLogId>,
}

#[main_codec]
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash,
)]
pub struct TxLogId {
    pub tx: TxNumber,
    pub block_number: BlockNumber,
    pub log: LogNumber,
}

impl From<(TxNumber, LogNumber, BlockNumber)> for TxLogId {
    fn from(tpl: (TxNumber, LogNumber, BlockNumber)) -> Self {
        TxLogId {
            tx: tpl.0,
            log: tpl.1,
            block_number: tpl.2,
        }
    }
}


impl Into<String> for TxLogId {
    fn into(self) -> String {
        format!{"{}_{}", self.tx, self.log}
    }
}