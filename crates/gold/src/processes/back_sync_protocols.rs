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
