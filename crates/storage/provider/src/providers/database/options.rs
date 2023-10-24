use rocksdb::DBCompressionType as RocksCompressionType;

pub struct DatabaseOptions {
    // The access type of blockstore. Default: Primary
    pub access_type: AccessType,
    // When opening the Blockstore, determines whether to error or not if the
    // desired open file descriptor limit cannot be configured. Default: true.
    pub enforce_ulimit_nofile: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessType {
    /// Primary (read/write) access; only one process can have Primary access.
    Primary,
    /// Secondary (read) access; multiple processes can have Secondary access.
    /// Additionally, Secondary access can be obtained while another process
    /// already has Primary access.
    Secondary,
}

#[derive(Debug, Clone)]
pub enum DatabaseCompressionType {
    None,
    Snappy,
    Lz4,
    Zlib,
}

impl Default for DatabaseCompressionType {
    fn default() -> Self {
        Self::None
    }
}

impl DatabaseCompressionType {
    #[allow(dead_code)]
    pub(crate) fn to_rocksdb_compression_type(&self) -> RocksCompressionType {
        match self {
            Self::None => RocksCompressionType::None,
            Self::Snappy => RocksCompressionType::Snappy,
            Self::Lz4 => RocksCompressionType::Lz4,
            Self::Zlib => RocksCompressionType::Zlib,
        }
    }
}