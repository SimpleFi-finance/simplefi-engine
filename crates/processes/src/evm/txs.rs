use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process};

pub struct TransactionsProcess;

impl Process for TransactionsProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Transactions
    }

    fn execute<T>(&mut self, db_provider: Option<&DatabaseProvider>) -> T {

        // load chain Rpc methods
        // get headers and return or store them
        unimplemented!()
    }
}