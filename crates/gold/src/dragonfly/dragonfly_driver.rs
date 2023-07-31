use chains_types::SupportedChains;
use data_lake_types::SupportedDataTypes;
use redis::{aio::Connection, RedisError};
use serde_json;
use simplefi_engine_settings::load_settings;
use simplefi_redis::{
    add_to_set, connect, delete_from_hset, delete_multiple_from_hset, get_complete_hset,
    get_from_hset, is_in_set, store_in_hset,
};

use crate::mongo::volumetrics::utils::shared::get_month_year_day_hour_minute;
use crate::protocol_driver::driver_traits::protocol_info::GetProtocolInfo;
// use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;
use crate::types::shared::Timeframe;
use crate::utils::volumetrics::amalgamate_volumetrics_vecs;

pub struct ProtocolDragonflyDriver {
    pub connection: Connection,
    pub chain: String,
}

impl ProtocolDragonflyDriver {
    pub async fn new(chain: &str) -> Self {
        let mysettings = load_settings().expect("Failed to load settings");
        let redis_connection = connect(&mysettings.redis_uri)
            .await
            .expect("Expect to connect to redis");

        Self {
            connection: redis_connection,
            chain: String::from(chain),
        }
    }

    pub fn resolve_collection_name(
        &self,
        data_type: SupportedDataTypes,
        timeframe: &Timeframe,
    ) -> String {
        format!(
            "{}_gold_{}_{}",
            &self.chain,
            data_type.to_string(),
            timeframe.timeframe_in_text()
        )
    }
    pub fn split_field_key(
        &self,
        key: &str,
    ) -> (String, u64) {
        let key_parts: Vec<&str> = key.split("-").collect();

        if key_parts.len() != 2 {
            panic!("Invalid key")
        }

        let key_ts: u64 = key_parts[1].parse().unwrap();
        (key_parts[0].clone().to_string(), key_ts)
    }

    // pub fn resolve_key_extension(
    //     &self,
    //     timeframe: &Timeframe,
    //     timestamp: &u64,
    // ) {
    //     let (year, month, day, hour, minute) = get_month_year_day_hour_minute(timestamp);

    //     match timeframe {
    //         Timeframe::FiveMinute => {
    //             format!("{}_{}_{}_{}_{}", &year,&month,&day,&hour, &minute)
    //         }
    //         Timeframe::Hourly => {
    //             format!("{}_{}_{}_{}", &year,&month,&day,&hour)
    //         }
    //         Timeframe::Daily => {
    //             format!("{}_{}_{}", &year,&month,&day)
    //         }
    //         _ => panic!("Timeframe not activated"),
    //     }
    // }
}
