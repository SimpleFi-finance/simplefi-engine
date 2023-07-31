use std::env;

use chains_types::get_chain;
use chrono::{Days, Utc};
use mongo_types::{Mongo, MongoConfig};

use crate::{
    mongo::protocol_status::{getters::get_all_protocols, types::ProtocolStatus},
    processes::backfill::process_from_parquet::process_from_parquet,
    protocol_driver::protocol_driver::SupportedProtocolDrivers,
};

pub async fn back_sync_protocols() {
    let chain_id = env::var("CHAIN_ID").unwrap();
    let chain = get_chain(&chain_id).unwrap();
    let config = MongoConfig {
        uri: "mongodb://localhost:27017".to_string(),
        database: "test".to_string(),
    };
    let db = Mongo::new(&config).await.unwrap();

    // Create a new MongoDB client
    let protocol_status = get_all_protocols(&db, &chain_id).await.unwrap();

    // filter all that are within threshold ( 1 hour)
    let threshold = Utc::now().timestamp_millis() - 3600000;
    let outdated_protocols = protocol_status
        .iter()
        .cloned()
        .filter(|x| !x.should_update || x.last_sync_block_timestamp < threshold)
        .collect::<Vec<ProtocolStatus>>();

    // find oldest update
    // let oldest_update = outdated_protocols.iter().reduce(threshold, |(acc, p)| {
    //     if p.last_sync_block_timestamp < acc {
    //         p.last_sync_block_timestamp
    //     }
    //     acc
    //     // p.last_sync_block_timestamp < acc ? p.last_sync_block_timestamp : acc
    // });
    let mut oldest_update = threshold;
    outdated_protocols.iter().for_each(|x| {
        if x.last_sync_block_timestamp < oldest_update {
            oldest_update = x.last_sync_block_timestamp;
        }
    });

    let last_updated_timestamp = oldest_update;
    // sync from parquet
    loop {
        // process_from_parquet().await;
        // get logs for for same day greater than last_updated_timestamp
    }

    // sync from mongo

    todo!();
}

/*
 Load all protocol status
 filter for all protocol status that were last updated < threshold timestamp (snapshot or volumetric)
 get lowest timestamp from protocol status' (snapshot or volumetric) for starting point

 let last_day_processed = lowest timestamp from protocol status


 // from parquet (possible stream), await
   get logs for that starting day that are greater than lowest timestamp (same day as timestamp)
     filter for logs that have address from factory address list
     bin by address
     for each factor address dataframe
       get the matching protocol driver
       get new market addresses from logs
       save market address using redis driver

     after processing factory logs:
       bin original df by address
       for each address
         check redis driver if address is in any of the sets
         if in set
           check protocol status for last timestamp checked, if older, do nothing
           if newer
              check redis if snapshots/volumetrics exists for that address
               if they do, check if same period (5 min, hour, day)
                 if same periods, use as base for new snapshots/volumetrics
                 if not same periods, create new snapshots to use as (using previous figures for snapshots), store previous periods in mongo and clean redis
                 use matched driver to process logs and create snapshots/volumetrics
                 update protocol status last updated timestamp

       update last_day_processed + 1

 // from mongo (possible stream)
   get lots from mongo starting from last_day_processed + 1
   bin by address into a hashmap
   get hashmap entries for logs that have address from factory address list
     for each factor address dataframe
       get the matching protocol driver
       get new market addresses from logs (method for getting it from mongo logs?)
       save market address using redis driver

    after processing factory logs:
       for each address in hashmap
       check redis driver if address is in any of the sets
       if in set
       check protocol status for last timestamp checked, if older, do nothing
       if newer
             normalize logs
             check redis if snapshots/volumetrics exists for that address
               if they do, check if same period (5 min, hour, day)
                 if same periods, use as base for new snapshots/volumetrics
                 if not same periods, create new snapshots to use as (using previous figures for snapshots), store previous periods in mongo and clean redis
                 use matched driver to process logs and create snapshots/volumetrics
                 save most recent 5min,1hour,1day snapshot/volumetric in redis for that address, save all older in mongo
                 update protocol status last updated timestamp

       update last_day_processed + 1


 once last_day_processed > than today, break

 update protocol status properties, changing shouldUpdate properties to true for protocols that have synced into the threshhold (should be all that have been syncing)



*/

/*
 notes:
   use centralised process for getting + sorting logs
   then have two seperate services (threads) to handle all volumetric/snapshot logic
   so not to overcrowd the main process

   design so that the volumetric and snapshot logic is reusable between the backfil and update processes
   both get given logs, redis driver etc.  They check driver for previous entries and create snapshots/volumetrics.  They then save in mongo/redis

   make sure to think about when to update protocol status timestamps.  E.g don't process address A, update status and then go to process a new address to see that the threshhold has changes
     possible solution:  update local variable for that protocol which then is used to update the the protocol status at the end of that day
*/

/*
   Overall structure for backfill:

     get protocol status
     iterate through days, getting logs from parquet
     if no logs from parquet, break
     else
       process factory address logs
       bin by address, iterate and call methods for volumetric and snapshots to process that market

     iterate through days (from last_day_processed), gettings logs from mongo
     process factory address logs
     bin by address, iterate and call methods for volumetric and snapshots to process that market

     update protocol status figures

*/
