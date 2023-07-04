/*
 Update protocols


   get all protocol status'
   find oldest last timestamp from protocol status' that is within the threshold (1 week/2 days??)

   get logs from mongo > that oldest last timestamp
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

*/

/*
   most of the logic here is reused from backfil logic.
   Only difference is this never checks parquet, doesn't iterate through days
   all processing logic should be reusable
*/
