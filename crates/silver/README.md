# Silver Pipeline

## IMPORTANT
Migration of the silver pipeline to the current state machine format has not been completed.
The previous version used Dragonfly (redis) and mongoDB to store the majority of it's defi data output, this is no longer the case.
Storage methods for rocksDB have been created and tested however not implemented fully.
Full migration of pipeline logic has also not been completed.


## About
The silver pipeline digests decoded log/event data from the bronze pipeline.  The events are processed for each market in each
protocol, creating volumetric and historical snapshot data.

This is achieved by tracking all creation events from protocol contracts, matching market addresses to prototocols.  Once
a market has been added, events from that creation block will be tracked and processed, e.g transfers, LP token swaps, owners etc.


There are two main parts:

### Backfill: 
This process goes through the blockchain history, looking at which protocol volumes/snapshots are out of sync, 
and bringing them update to date.  

### Update:  
This process reacts to new blocks/events from the bronze deadline, updating protocol markets volumes/snapshots.

### Aggreation and migration:  
The Update process create 5 minute volumes and snapshots for each given market, stored in Dragonfly (Redis).  Each hour there is an aggregation stage fired off, which retrieves this volumes/snapshots and creates an hourly volume/snapshot. At this point all 5 minute snapshots are migration to long-storage (our embedded RocksDB database).  If this stage is fired off at the end of a given day, the hourly volumes/snapshots will be aggregated into a daily volume/snapshot and all hourly ones will be migrated to long-term storage alongside the newly created daily entry.

* There is a timestamp for each protocol, showcasing when that protocol was last updated/checked for updates (even if there aren't any events for a specific protocol in a given block, the protocol will still update it's updated at timestamp to that time).  The "Update" process uses a threshold (e.g 2 days ago) and retrieve events within that threshold timeframe for markets that are up until that point.  
The Backfil process will update protocols to current time, pushing new and out of sync protocols into the recent threshold and therefore to be picked up by the update process.
Update and Backfill processes are frequently started to ensure up to date data.

## Protocols driver

The protocol driver is a Rust Struct with defined traits to manage protocols.  To track new protocols, we will need to create the needed trait methods for new protocol and point to them in the origin Protocol driver methods.

As each protocol runs unique (mostly) smart contracts with different events, custom implementation is needed to process each protocols events.  Therefore we have extrapulated this requirement from the main process by creating this driver, allowing us to handle the the rest of the process protocol agnostic. 

## Protocol Status

Each protocol is tracked with a protocol status entry.  This simple document shows if the protocol is synced, it's last active blockand if there have been any errors.

## Process Output Data 

As mentioned the silver pipeline outputs volumetric and snapshot data for protocol markets, in three period types, 5 minute, hourly, daily.

### Volumetrics

Contains volumetric data for a given market for a given period (e.g 5 minutes).  Totals of all swaps, mints,burns, withdrawals and transfers.

```
pub struct AddressBalance {
  pub address: H256,
  pub balance: H256
}

pub struct Volumetric {
    pub timestamp: u64,
    pub swaps_out: Vec<AddressBalance>,  
    pub swaps_in: Vec<AddressBalance>,   
    pub withdrawal: Vec<AddressBalance>, 
    pub mint: Vec<AddressBalance>,       
    pub transfer: H256,        
}
```

### Snapshots

// TODO!










