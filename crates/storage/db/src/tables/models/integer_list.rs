//! Implements [`Compress`] and [`Decompress`] for [`IntegerList`]

use crate::table::{Compress, Decompress};
use interfaces::db::DatabaseError;
use simp_primitives::IntegerList;

impl Compress for IntegerList {
    type Compressed = Vec<u8>;

    fn compress(self) -> Self::Compressed {
        self.to_bytes()
    }
    fn compress_to_buf<B: bytes::BufMut + AsMut<[u8]>>(self, buf: &mut B) {
        self.to_mut_bytes(buf)
    }
}

impl Decompress for IntegerList {
    fn decompress<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
        IntegerList::from_bytes(value.as_ref()).map_err(|_| DatabaseError::DecodeError)
    }
}
