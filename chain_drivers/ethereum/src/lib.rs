pub mod block_data_indexer;
pub mod blocks_tracker;
pub mod logs_tracker;
pub mod types;
pub mod chain;
pub mod utils;
// listens to new blocks and saves data to database

//syncs historical data from a given block to the latest block


// driver should include the db saving methods, db reading and data-lake methods for db to bronze?

// data-lake should be a single module to read and gather data independently of chain drivers
// inject the driver methods to the data-lake functions to read and write data from the parquet files

// or should it be a separate module?


