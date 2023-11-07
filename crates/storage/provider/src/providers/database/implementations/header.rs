use db::tables::Headers;
use interfaces::Result;
use simp_primitives::{BlockHash, Header, BlockNumber};
use rocksdb::ReadOptions;
use db::tables::utils::decoder;

use crate::DatabaseProvider;
use crate::traits::{BlockNumReader, HeaderProvider, HeaderWriter};
use db::table::Encode;
use db::transaction::DbTx;

impl HeaderProvider for DatabaseProvider {

    fn header(&self, block_hash: &BlockHash) -> Result<Option<Header>> {
        if let Some(num) = self.block_number(*block_hash)? {
            Ok(self.header_by_number(num)?)
        } else {
            Ok(None)
        }
    }

    fn header_by_number(&self, num: BlockNumber) -> Result<Option<Header>> {
        let header = self.db.dae_get::<Headers>(num).unwrap();
        Ok(header)
    }

    // It will always include the range bounds
    fn headers_range(&self, range: (BlockNumber, BlockNumber)) -> Result<Vec<Header>> {
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(range.0.encode().as_slice()..range.1.encode().as_slice());
        let mut iter = self.db.dae_new_cursor::<Headers>(opts).unwrap();
        let mut headers = Vec::new();
        iter.seek_to_first();

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv = decoder::<Headers>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
            headers.push(kv.1);
            iter.next();
        }

        Ok(headers)
    }

    fn latest_header(&self) -> Result<Option<Header> > {
        let mut iter = self.db.dae_new_cursor::<Headers>(ReadOptions::default()).unwrap();
        iter.seek_to_last();

        if iter.valid() {
            let decoded = decoder::<Headers>(
                (
                    iter.key().unwrap().to_vec(),
                    iter.value().unwrap().to_vec()
                )
            ).unwrap();

            Ok(Some(decoded.1))
        } else {
            Ok(None)
        }
    }
}

impl HeaderWriter for DatabaseProvider {
    fn insert_header(&self, block_number: &BlockNumber, header: Header) -> Result<Option<BlockNumber> > {
        self.db.dae_put::<Headers>(*block_number, header)?;
        Ok(Some(*block_number))
    }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use db::{init_db, test_utils::ERROR_TEMPDIR};
    use simp_primitives::Header;
    use crate::traits::{HeaderProvider, HeaderWriter, BlockNumWriter, BlockHashWriter};
    use crate::DatabaseProvider;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path()).unwrap();

        DatabaseProvider::new(db, crate::providers::options::AccessType::Primary)
    }

    fn get_mocks_headers() -> String {
        let path = "./src/mocks/mocks_headers.json";
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn insert_and_retrieve_header_by_bn() {
        let headers = get_mocks_headers();

        let headers: Vec<Header> = serde_json::from_str(&headers).unwrap();

        let provider = get_provider();

        for header in headers {
            provider.insert_header(&header.number, header.clone()).unwrap();
            let header_from_db = provider.header_by_number(header.number).unwrap().unwrap();
            assert_eq!(header_from_db, header);
        }
    }

    #[test]
    fn insert_and_retrieve_header_by_hash() {
        let headers = get_mocks_headers();

        let headers: Vec<Header> = serde_json::from_str(&headers).unwrap();

        let provider = get_provider();

        for header in headers {
            provider.insert_header(&header.number, header.clone()).unwrap();
            provider.insert_block_number(header.hash, header.number).unwrap();
            provider.insert_block_hash(header.number, header.hash).unwrap();

            let header_from_db = provider.header(&header.hash()).unwrap().unwrap();
            assert_eq!(header_from_db, header);
        }
    }


    #[test]
    fn insert_and_retrieve_header_range() {
        let headers = get_mocks_headers();

        let headers: Vec<Header> = serde_json::from_str(&headers).unwrap();

        let provider = get_provider();
        let bns = headers.clone().iter().map(|h| h.number).collect::<Vec<_>>();
        let min = bns.iter().min().unwrap();
        let max = bns.iter().max().unwrap();

        for header in headers.clone() {
            provider.insert_header(&header.number, header.clone()).unwrap();
        }

        let headers_from_db = provider.headers_range((*min, *max + 1)).unwrap();
        assert_eq!(headers_from_db.len(), headers.len());
        assert_eq!(headers_from_db[0].hash, headers[0].hash);
    }
}