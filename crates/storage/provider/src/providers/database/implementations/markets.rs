use crate::DatabaseProvider;
use crate::traits::MarketProvider;
use db::tables::{ MarketProtocol, TokensMarkets};
use interfaces::Result;
// use rocksdb::ReadOptions;
// use db::tables::utils::decoder;
use db::transaction::DbTx;
// use db::table::Encode;
// use db::tables::models::VolumeKeysWithData;
use primitives::{H256, Market, TokenMarkets};



impl MarketProvider for DatabaseProvider {
    fn create_market(&self, market: Market, market_address: H256) -> Result<()> {
        self.db.put::<MarketProtocol>(market_address, market.clone())?;
        for token in market.input_tokens {
            self.add_to_token_markets(market_address, token)?;
        }
        
        Ok(())
    }

    fn delete_market(&self, market_address: H256) -> Result<()> {
        self.db.delete::<MarketProtocol>(market_address)?;

        Ok(())
    }

    fn get_market(&self, market_address: H256) -> Result<Option<Market>> {
        let market = self.db.get::<MarketProtocol>(market_address)?;
        Ok(market)
    }

    fn update_market(&self, market_address: H256, updated_market: Market) -> Result<()> {
        let matched_market = self.db.get::<MarketProtocol>(market_address)?;

        match matched_market {
            Some(_) => self.db.put::<MarketProtocol>(market_address,updated_market)?,
            _ => ()
        }

        Ok(())
    }

    fn add_to_token_markets (&self, market_address: H256, token_address: H256) -> Result<()> {
        let token = self.db.get::<TokensMarkets>(token_address)?;
        match token {
            Some(mut t) => {
                t.market_addresses.push(market_address);
                self.db.put::<TokensMarkets>(token_address,t)?;
            },
            _ => {
                self.db.put::<TokensMarkets>(token_address,TokenMarkets {market_addresses: vec![market_address]})?;
            }
        }
        Ok(())
    }
    fn get_token_markets(&self, token_address: H256) -> Result<Option<TokenMarkets>> {
        let t = self.db.get::<TokensMarkets>(token_address)?;
        Ok(t)
    }
}



#[cfg(test)]
mod test {
    use crate::traits::MarketProvider;
    use crate::{providers::options::AccessType, DatabaseProvider};
    use db::{
        implementation::sip_rocksdb::DB, init_db, 
        test_utils::ERROR_TEMPDIR,
    };
    use primitives::{H256, Market};
    use std::str::FromStr;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        DatabaseProvider::new(db, AccessType::Primary)
    }


    #[test]
    fn insert_and_retrieve() {
        let provider = get_provider();
        let new_market = Market {
            protocol_id: 1,
            input_tokens: vec![H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap()],
        };
        provider.create_market(new_market, H256::zero()).expect("Expect to create market");
        let fetched_market = provider.get_market(H256::zero()).expect("Expect to retrieve market");
        assert!(fetched_market.is_some());
        match fetched_market {
            Some(m) => assert!(m.protocol_id == 1),
            _ => panic!()
        }
    }

    #[test]
    fn insert_and_retrieve_delete() {
        let provider = get_provider();
        let new_market = Market {
            protocol_id: 1,
            input_tokens: vec![H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap()],
        };
        provider.create_market(new_market, H256::zero()).expect("Expect to create market");
        let fetched_market = provider.get_market(H256::zero()).expect("Expect to retrieve market");
        assert!(fetched_market.is_some());
        match fetched_market {
            Some(m) => assert!(m.protocol_id == 1),
            _ => panic!()
        }

        let _ = provider.delete_market(H256::zero());

        let fetched_market2 = provider.get_market(H256::zero()).expect("Expect to retrieve market");
       
        match fetched_market2 {
            None => assert!(true),
            _ => panic!()
        }
    }
    
    #[test]
    fn token_markets() {
        let provider = get_provider();
        let new_market = Market {
            protocol_id: 1,
            input_tokens: vec![H256::default()],
        };
      
        provider.create_market(new_market.clone(), H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap()).expect("Expect to create market");
        provider.create_market(new_market.clone(), H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap()).expect("Expect to create market");
        
        let token_markets = provider.get_token_markets(new_market.input_tokens[0]).expect("Expect to retrieve token markets");

        match token_markets {
            Some(markets) => assert!(markets.market_addresses.len() == 2),
            _ => panic!()
        }
    }
}