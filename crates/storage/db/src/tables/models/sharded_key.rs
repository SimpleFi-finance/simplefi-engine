//! Sharded key

use std::hash::Hash;

use crate::table::{Decode, Encode};
use serde::{Deserialize, Serialize};
use interfaces::db::DatabaseError;
/// Number of indices in one shard.
pub const NUM_OF_INDICES_IN_SHARD: usize = 10_000;

/// Sometimes data can be too big to be saved for a single key. This helps out by dividing the data
/// into different shards. Example:
///
/// `Address | 200` -> data is from block 0 to 200.
///
/// `Address | 300` -> data is from block 201 to 300.
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ShardedKey<T> {
    /// The key for this type.
    pub key: T,
    /// Highest value to which `value` is related to.
    pub max_shard_value: u64,
}

impl<T> AsRef<ShardedKey<T>> for ShardedKey<T> {
    fn as_ref(&self) -> &ShardedKey<T> {
        self
    }
}

impl<T> ShardedKey<T> {
    /// Creates a new `ShardedKey<T>`.
    pub fn new(key: T, highest_value: u64) -> Self {
        ShardedKey { key, max_shard_value: highest_value }
    }

    /// Creates a new key with the highest value set to maximum.
    /// This is useful when we want to search the last value for a given key.
    pub fn last(key: T) -> Self {
        Self { key, max_shard_value: u64::MAX }
    }
}

impl<T> ShardedKey<T> {
    pub fn get_shard_from_value(&self, value: u64) -> u64 {
        if value == 0 {
            return self.max_shard_value as u64;
        } else {
            if value <= self.max_shard_value {
                return self.max_shard_value;
            } else {
                let times = self.max_shard_value / value;

                let modulus = self.max_shard_value % value;

                if modulus == 0 {
                    return self.max_shard_value * times;
                } else {
                    return (times + 1) * value;
                }
            }

        }
    }   
}

impl<T> Encode for ShardedKey<T>
where
    T: Encode,
    Vec<u8>: From<<T as Encode>::Encoded>,
{
    type Encoded = Vec<u8>;

    fn encode(self) -> Self::Encoded {
        let mut buf: Vec<u8> = Encode::encode(self.key).into();
        buf.extend_from_slice(&self.max_shard_value.to_be_bytes());
        buf
    }
}

impl<T> Decode for ShardedKey<T>
where
    T: Decode,
{
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
        let value = value.as_ref();

        let tx_num_index = value.len() - 8;

        let highest_value = u64::from_be_bytes(
            value[tx_num_index..].try_into().map_err(|_| DatabaseError::DecodeError)?,
        );
        let key = T::decode(&value[..tx_num_index])?;

        Ok(ShardedKey::new(key, highest_value))
    }
}

impl<T> Hash for ShardedKey<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.max_shard_value.hash(state);
    }
}
