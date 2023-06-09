/*
    checks the blocks missing from partitions in bucket
    if no blocks present starts from zero and calculates the approximate block number from 0 in partition intervals using the averageblocktime. 
    pushes the ranges to a list/pubsub
    after doing so, backfills the partition of current data from latest block available to beginning of partition and stores data in mongo current
 */

use chains_drivers::ethereum::mainnet::ethereum_mainnet;
use processes::settings::load_settings;

#[tokio::main]

async fn main() {

    todo!("Implement backfill process");

    let settings = load_settings().expect("Failed to load settings");

    let chain = match settings.chain_id.as_str() {
        "1" => ethereum_mainnet().await.unwrap(),
        _ => panic!("Chain not supported"),
    };


    /*

    boundaries of partition (approximate)
        blocks_per_partition = Partition_seconds / Blocktime_average

        bn_p0 = bn_0 + (blocks_per_partition * partition_number ) 

        bn_p1 = bn_0 + (blocks_per_partition * partition_number) + blocks_per_partition
     
     from bn_p0 and bn_p1 find the correct block numbers to create ranges and fill the list/pubsub
    */


    todo!("implement last partition backfill");
}