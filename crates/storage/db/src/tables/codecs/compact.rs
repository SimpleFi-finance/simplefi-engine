use crate::{
    table::{Compress, Decompress},
    tables::models::*,
};
use sip_codecs::{main_codec, Compact};
use simp_primitives::*;
use interfaces::db::DatabaseError;
use simp_primitives::Volumetric;

/// Implements compression for Compact type.
macro_rules! impl_compression_for_compact {
    ($($name:tt),+) => {
        $(
            impl Compress for $name
            {
                type Compressed = Vec<u8>;

                fn compress_to_buf<B: bytes::BufMut + AsMut<[u8]>>(self, buf: &mut B) {
                    let _  = Compact::to_compact(self, buf);
                }
            }

            impl Decompress for $name
            {
                fn decompress<B: AsRef<[u8]>>(value: B) -> Result<$name, DatabaseError> {
                    let value = value.as_ref();
                    let (obj, _) = Compact::from_compact(&value, value.len());
                    Ok(obj)
                }
            }
        )+
    };
}

impl_compression_for_compact!(
    Header,
    Log,
    AbiData,
    TxLogs,
    TxLogId,
    StoredLog,
    StoredDecodedData,
    ContractData,
    TxType,
    StoredContract,
    BlockBodyIndices,
    TransactionSigned,
    CompactU256,
    VolumeKeyWithData,
    Volumetric,
    VolumeKeys,
    VolumeKeysWithData,
    Protocol,
    Market,
    TokenMarkets,
    PeriodVolumes
);

macro_rules! impl_compression_fixed_compact {
    ($($name:tt),+) => {
        $(
            impl Compress for $name
            {
                type Compressed = Vec<u8>;

                fn compress_to_buf<B: bytes::BufMut + AsMut<[u8]>>(self, buf: &mut B) {
                    let _  = Compact::to_compact(self, buf);
                }

                fn uncompressable_ref(&self) -> Option<&[u8]> {
                    Some(self.as_ref())
                }
            }

            impl Decompress for $name
            {
                fn decompress<B: AsRef<[u8]>>(value: B) -> Result<$name, DatabaseError> {
                    let value = value.as_ref();
                    let (obj, _) = Compact::from_compact(&value, value.len());
                    Ok(obj)
                }
            }

        )+
    };
}

impl_compression_fixed_compact!(H256, H160);

/// Adds wrapper structs for some primitive types so they can use StructFlags from Compact, when
/// used as pure table values.
macro_rules! add_wrapper_struct {
    ($(($name:tt, $wrapper:tt)),+) => {
        $(
            /// Wrapper struct so it can use StructFlags from Compact, when used as pure table values.
            #[main_codec]
            #[derive(Debug, Clone, PartialEq, Eq, Default)]
            pub struct $wrapper(pub $name);

            impl From<$name> for $wrapper {
                fn from(value: $name) -> Self {
                    $wrapper(value)
                }
            }

            impl From<$wrapper> for $name {
                fn from(value: $wrapper) -> Self {
                    value.0
                }
            }

            impl std::ops::Deref for $wrapper {
                type Target = $name;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

        )+
    };
}

add_wrapper_struct!((U256, CompactU256));
add_wrapper_struct!((u64, CompactU64));
