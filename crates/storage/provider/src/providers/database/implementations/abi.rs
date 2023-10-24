use std::collections::HashMap;
use db::tables::models::{ContractData, AbiContract};
use db::tables::utils::decoder;
use db::tables::{self, MarketToProxy, ContractsData, ContractProxy, Abi, AbiData, UnknownContracts};
use db::transaction::DbTx;
use ethabi::ethereum_types::H256;
use primitives::{Address, StoredLog, DecodedData};
use interfaces::Result;
use rocksdb::ReadOptions;
use crate::DatabaseProvider;
use crate::traits::{AbiProvider, AbiWriter};
use ethabi::{Contract, RawLog, Event, Token};

enum TokenType {
    Bool,
    String,
    Address,
    Bytes,
    Int,
    Array,
    Tuple,
}

impl Into<u64> for TokenType {
    fn into(self) -> u64 {
        match self {
            TokenType::Bool => 0,
            TokenType::String => 1,
            TokenType::Address => 2,
            TokenType::Bytes => 3,
            TokenType::Int => 4,
            TokenType::Array => 5,
            TokenType::Tuple => 6,
        }
    }
}

struct TokenWType {
    value: String,
    token_type: TokenType,
}

fn get_token_type(token: Token) -> Result<TokenWType> {

    Ok(match token {
        Token::Bool(token) => TokenWType {
            token_type: TokenType::Bool,
            value: token.to_string(),
        },
        Token::String(token) => TokenWType {
            token_type: TokenType::String,
            value: token.to_string(),
        },
        Token::Address(token) => TokenWType {
            token_type: TokenType::Address,
            value: format!("0x{:x}", token),
        },
        Token::Bytes(token) | Token::FixedBytes(token) => TokenWType {
            token_type:TokenType::Bytes,
            value: serde_json::to_string(&token).unwrap(),
        },
        Token::Uint(token) | Token::Int(token) => TokenWType {
            token_type: TokenType::Int,
            value: token.to_string(),
        },
        Token::Array(token) | Token::FixedArray(token) => TokenWType {
            token_type: TokenType::Array,
            value: serde_json::to_string(&token).unwrap(),
        },
        Token::Tuple(token) => TokenWType {
            token_type: TokenType::Tuple,
            value: serde_json::to_string(&token).unwrap(),
        },
    })
}

impl AbiProvider for DatabaseProvider {
    fn decode_logs(&self, logs: Vec<StoredLog>, abi: &Vec<AbiContract>) -> Result<Vec<Option<Vec<DecodedData>>>> {
        // TODO: make sure to use only the correct abi and not past abis
        // TODO: this is only relatable to evms, should find a better way to inject different decoding behaviours

        let mut hm_events: HashMap<H256, Event> = HashMap::new();

        abi.iter().for_each(|a|{
            let a: Contract = serde_json::from_slice(&a.abi.body).unwrap();
            a.events().for_each(|e| {
                hm_events.entry(e.signature()).or_insert(e.clone());
            });
        });


        let raw_logs = logs.iter().map(|log| {

            let topics = log.topics.iter().map(|t| {
                H256::from(*t)
            }).collect::<Vec<H256>>();

            RawLog {
                topics,
                data: log.data.to_vec()
            }
        }).collect::<Vec<RawLog>>();

        let mut decoded_logs = Vec::new();
        // each log has multiple decoded data fields

        for raw_log in raw_logs {

            let event = hm_events.get(&raw_log.topics[0]);

            if event.is_some() {
                let decoded_data = event.unwrap().parse_log(raw_log.clone()).unwrap();

                let decoded_data = decoded_data.params.iter().enumerate().map(|(i,d)| {
                                    
                    let token_type = get_token_type(d.value.clone()).unwrap();

                    DecodedData {
                        name: d.name.clone().as_bytes().to_vec(),
                        value: token_type.value.as_bytes().to_vec(),
                        kind: token_type.token_type.into(),
                        indexed: event.unwrap().inputs[i].indexed,
                        signature: raw_log.topics[0].into(),
                    }
                }).collect::<Vec<DecodedData>>();

                decoded_logs.push(Some(decoded_data));
            } else {
                println!("missing event {:?}", raw_log.topics[0]);
                decoded_logs.push(None);
            }
        }

        Ok(decoded_logs)
    }

    fn get_abi_by_id(&self, id: u64) -> Result<Option<AbiData>> {
        let abi = self.db.get::<Abi>(id).unwrap();

        Ok(Some(abi.unwrap()))
    }

    fn get_proxy_data(&self, address: Address) -> Result<Option<tables::StoredContract>> {
        let proxy_data = self.db.get::<ContractProxy>(address).unwrap();

        Ok(proxy_data)
    }

    fn get_abis_by_address(&self, address: Address) -> Result<Option<Vec<AbiContract>>> {

        // check if we found the abi
        let missing_abi = self.address_without_abi(address).unwrap();

        if missing_abi.is_some() {
            return Ok(None);
        }
        // check if proxy
        let is_proxy = self.has_proxy(address).unwrap();

        match is_proxy {
            Some(proxy) => {
                let contracts = self.get_proxy_data(proxy).unwrap();
                let proxy_abi = self.get_abi_by_id(contracts.clone().unwrap().abi_id).unwrap();
                let mut abis = vec![];  

                abis.push(AbiContract { 
                    address, 
                    abi: proxy_abi.unwrap(), 
                    contract_type: String::from("proxy") 
                });

                for contract in contracts.unwrap().implementations {
                    let address = self.get_contract_data(contract.address).unwrap();

                    let abi = self.get_abi_by_id(address.unwrap().abi_id).unwrap();
                    
                    abis.push(AbiContract { address: contract.address, abi: abi.unwrap(), contract_type: String::from("contract") });
                }
                Ok(Some(abis))
            },
            None => {
                let contract_data = self.get_contract_data(address).unwrap();
                match contract_data {
                    Some(contract) => {
                        let abi = self.get_abi_by_id(contract.abi_id).unwrap();

                        Ok(Some(vec![AbiContract {
                            address,
                            contract_type: String::from("contract"),
                            abi: abi.unwrap()
                        }]))
                    },
                    None => Ok(None)
                }
            }
        }
    }

    fn get_contract_data(&self, address: Address) -> Result<Option<ContractData>> {
        let data = self.db.get::<ContractsData>(address).unwrap();
        Ok(data)
    }

    fn has_proxy(&self, address: Address) -> Result<Option<Address>> {
        let proxy = self.db.get::<MarketToProxy>(address).unwrap();
        Ok(proxy)
    }

    fn get_latest_abi(&self) -> Result<Option<u64>> {
        let opts = ReadOptions::default();

        let mut iter = self.db.new_cursor::<Abi>(opts)?;

        iter.seek_to_last();

        if iter.valid() {
            let k = iter.key().unwrap();
            let v = iter.value().unwrap();

            let (key, _val) = decoder::<Abi>((k.to_vec(), v.to_vec())).unwrap();

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn address_without_abi(&self, address: Address) -> Result<Option<(Address, u32)> > {
        let ts = self.db.get::<UnknownContracts>(address).unwrap();
        match ts {
            Some(ts) => {
                Ok(Some((address, ts)))
            },
            None => Ok(None)
        }
    }
}

impl AbiWriter for DatabaseProvider {
    fn insert_abi(&self, abi: AbiData) -> Result<Option<u64>> {
        let latest_abi = self.get_latest_abi().unwrap();
        // TODO: find a way to check for abi uniqueness (maybe by hash)
        let new_abi_id = match latest_abi {
            Some(id) => id + 1,
            None => 0
        };
        
        self.db.put::<Abi>(new_abi_id, abi)?;

        Ok(Some(new_abi_id))
    }

    fn insert_contract(&self, address: Address, abi_id: u64, verified: bool, block_number: Option<primitives::BlockNumber>) -> Result<Address> {
        let contract_data = ContractData::new(abi_id, block_number, verified);
        self.db.put::<ContractsData>(address.clone(), contract_data)?;
        
        Ok(address)
    }

    fn insert_contract_proxy_index(&self, address: Address, proxy: Address, force_upsert:bool) -> Result<Address> {

        let exists = self.has_proxy(address).unwrap();

        match exists {
            Some(p) => {
                if p != proxy && force_upsert {
                    if force_upsert {
                        self.db.put::<MarketToProxy>(address, proxy)?;
                    } else {
                        println!("address {:?}, mismatch proxy, existing {:?}, new {:?}, force update needs activation if update required", address, exists.unwrap(), proxy);
                    }
                }
                Ok(address)
            },
            None => {
                self.db.put::<MarketToProxy>(address, proxy)?;
                Ok(address)
            }
        }
    }

    fn upsert_proxy(&self, address: Address, abi_id: u64, verified: bool, implementations: Vec<tables::models::ProxyImplementations>) -> Result<Address> {
        let exists = self.get_proxy_data(address).unwrap();

        match exists {
            Some(p) => {    
                // get all prev implementations, update proxy, save
                let mut existing_impl = p.implementations;

                existing_impl.extend(implementations);

                let proxy_updated = tables::models::StoredContract::new(abi_id, verified, Some(existing_impl));
                self.db.put::<ContractProxy>(address, proxy_updated)?;

                Ok(address)
            },
            None => {
                let proxy = tables::models::StoredContract::new(abi_id, verified, Some(implementations));
                self.db.put::<ContractProxy>(address, proxy)?;

                Ok(address)
            }
        }
    }

    fn insert_unknown_contract(&self,address:Address,timestamp:u32) -> Result<()> {
        self.db.put::<UnknownContracts>(address, timestamp)?;
        Ok(())
    }
}



#[cfg(test)]
mod test {
    use std::fs;

    use chrono::Utc;
    use primitives::{H256, Address, H160, Log, StoredLog};
    use db::{tables::{AbiData, models::ProxyImplementations}, init_db, test_utils::ERROR_TEMPDIR, implementation::sip_rocksdb::DB};
    use hex_literal::hex;
    use crate::{DatabaseProvider, providers::options::AccessType, traits::{AbiWriter, AbiProvider}};

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        DatabaseProvider::new(db, AccessType::Primary)
    }

    fn get_uni_factory_abi() -> String {
        let path = "./src/mocks/uni_v2_factory.json";
        fs::read_to_string(path).unwrap()
    }

    fn get_uni_factory_logs() -> String {
        let path = "./src/mocks/uni_factory_logs.json";
        fs::read_to_string(path).unwrap()
    }

    fn get_uni_factory_market() -> String {
        let path = "./src/mocks/uni_v2_market.json";
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_insert_abi() {
       
        let abi_bytes = serde_json::to_vec(&get_uni_factory_abi()).unwrap();

        let abi = AbiData {
            hash: H256::default(),
            body: abi_bytes
        };

        let abi_decod: String = serde_json::from_slice(&abi.body).unwrap();

        assert_eq!(abi_decod, get_uni_factory_abi());

        let provider = get_provider();
        

        let id = provider.insert_abi(abi.clone()).unwrap();

        let abi_data = provider.get_abi_by_id(id.unwrap()).unwrap();

        assert_eq!(abi_data.unwrap(), abi);

    }

    #[test]
    fn test_insert_contract() {
        let provider = get_provider();
        let address = Address::from(1);
        let _ = provider.insert_contract(address, 1, false, None).unwrap();
    }

    #[test]
    fn test_insert_and_get_contract_data() {
        let provider = get_provider();

        let address = Address::from(1);

        let abi = get_uni_factory_abi();

        let address_2 = Address::from(2);

        let abi_2 = get_uni_factory_market();

        let abi_id_factory = provider.insert_abi(AbiData {
            hash: H256::default(),
            body: abi.as_bytes().to_vec()
        }).unwrap().unwrap();

        let abi_2_id = provider.insert_abi(AbiData {
            hash: H256::default(),
            body: abi_2.as_bytes().to_vec()
        }).unwrap().unwrap();

        assert_eq!(abi_id_factory, 0);
        assert_eq!(abi_2_id, 1);

        let _ = provider.insert_contract(address, abi_id_factory.clone(), false, None).unwrap();

        let _ = provider.insert_contract(address_2, abi_2_id.clone(), false, None).unwrap();

        let contract_data = provider.get_contract_data(address).unwrap();
        assert!(contract_data.is_some());
        let data = contract_data.unwrap();

        assert_eq!(data.abi_id, abi_id_factory);
        assert_eq!(data.verified, false);
        assert_eq!(data.block_number, None);
    }   

    #[test]
    fn test_insert_and_resolve_proxy_index() {
        let address_1 = Address::from(1);
        let proxy_1 = Address::from(3);
        let proxy_2 = Address::from(4);

        let provider = get_provider();

        provider.insert_contract_proxy_index(address_1, proxy_1, false).unwrap();
        
        let p_index = provider.has_proxy(address_1).unwrap();
        assert_eq!(p_index.unwrap(), proxy_1);
    
        provider.insert_contract_proxy_index(address_1, proxy_2, false).unwrap();

        let p_index = provider.has_proxy(address_1).unwrap();
        assert_eq!(p_index.unwrap(), proxy_1);

        provider.insert_contract_proxy_index(address_1, proxy_2, true).unwrap();

        let p_index = provider.has_proxy(address_1).unwrap();
        assert_eq!(p_index.unwrap(), proxy_2);
    
    }

    #[test]
    fn test_upsert_proxy () {
        let provider = get_provider();
        let proxy = Address::from(1);

        let proxy_implementations = vec![
            ProxyImplementations {
                address: Address::from(2),
                block_number: 10,
            }
        ];

        provider.upsert_proxy(proxy, 1, false, proxy_implementations.clone()).unwrap();

        let proxy_data = provider.get_proxy_data(proxy).unwrap();

        assert_eq!(proxy_data.clone().unwrap().abi_id, 1);
        assert_eq!(proxy_data.clone().unwrap().implementations, proxy_implementations);

        let new_impl = ProxyImplementations {
            address: Address::from(3),
            block_number: 20,
        };

        provider.upsert_proxy(proxy, 1, false, vec![new_impl.clone()]).unwrap();  

        let new_proxy_data = provider.get_proxy_data(proxy).unwrap();
        assert_ne!(new_proxy_data.clone().unwrap().implementations, proxy_data.clone().unwrap().implementations);

        assert_eq!(new_proxy_data.clone().unwrap().implementations[0], proxy_data.clone().unwrap().implementations[0]);
        assert_eq!(new_proxy_data.clone().unwrap().implementations[1], new_impl);
    }

    #[test]
    fn test_decode_logs() {
        let provider = get_provider();

        let factory_abi = get_uni_factory_abi();

        let abi_id = provider.insert_abi(AbiData {
            hash: H256::default(),
            body: factory_abi.as_bytes().to_vec()
        }).unwrap().unwrap();

        let uni_address = H160(hex!("5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f"));

        let _ = provider.insert_contract(uni_address, abi_id, false, None).unwrap();
        let mock_logs = get_uni_factory_logs();
        
        let logs = serde_json::from_str::<Vec<Log>>(&mock_logs).unwrap();

        let stored_logs = logs.iter().map(|l| {
            StoredLog::from(l.clone())
        }).collect::<Vec<StoredLog>>();

        let abi = provider.get_abis_by_address(uni_address).unwrap().unwrap();

        let decoded_logs = provider.decode_logs(stored_logs, &abi).unwrap();
        
        assert_eq!(decoded_logs.len(), logs.len());
        assert_eq!(decoded_logs[0].is_some(), true);
    }

    #[test]
    fn test_get_abi_by_id() {
        let provider = get_provider();

        let abi = get_uni_factory_abi();

        let abi_id = provider.insert_abi(AbiData {
            hash: H256::default(),
            body: abi.as_bytes().to_vec()
        }).unwrap().unwrap();

        let abi_data = provider.get_abi_by_id(abi_id).unwrap();

        assert_eq!(abi_data.unwrap().body, abi.as_bytes().to_vec());
    }

    #[test]
    fn test_get_abis_by_address() {
        let provider = get_provider();

        let address = Address::from(1);

        let abi = get_uni_factory_abi();

        let address_2 = Address::from(2);

        let abi_data = AbiData { hash: H256::default(), body: abi.as_bytes().to_vec() };
        let abi_id = provider.insert_abi(abi_data.clone()).unwrap();

        provider.insert_contract(address, abi_id.unwrap(), false, None).unwrap();

        let abi = provider.get_abis_by_address(address).unwrap();

        assert!(abi.is_some());

        assert_eq!(abi.clone().unwrap()[0].address, address);
        assert_eq!(abi.unwrap()[0].abi, abi_data);

        let missing_abi = provider.get_abis_by_address(address_2).unwrap();

        assert!(missing_abi.is_none());
    }
    
    #[test]
    fn test_get_latest_abi() {
        let provider = get_provider();

        let abi = get_uni_factory_abi();

        let abi_id = provider.insert_abi(AbiData {
            hash: H256::default(),
            body: abi.as_bytes().to_vec()
        }).unwrap().unwrap();

        let latest_abi_id = provider.get_latest_abi().unwrap();

        assert!(latest_abi_id.is_some());
        assert!(latest_abi_id.unwrap() == abi_id);

        provider.insert_abi(AbiData {
            hash: H256::default(),
            body: abi.as_bytes().to_vec()
        }).unwrap().unwrap();

        let abi_id = provider.insert_abi(AbiData {
            hash: H256::default(),
            body: abi.as_bytes().to_vec()
        }).unwrap().unwrap();

        let latest_abi_id = provider.get_latest_abi().unwrap();

        assert!(latest_abi_id.is_some());
        assert!(latest_abi_id.unwrap() == abi_id);
        assert_eq!(latest_abi_id.unwrap(), 2);

    }


    #[test]
    fn test_insert_and_retrieve_unknown_contract() {
        let address = Address::from(1);

        let provider = get_provider();

        let address_2 = Address::from(2);
        let ts = Utc::now().timestamp() as u32;

        provider.insert_unknown_contract(address, ts).unwrap();

        let unknown_contract = provider.address_without_abi(address).unwrap();

        assert!(unknown_contract.is_some());

        let known_contract = provider.address_without_abi(address_2).unwrap();

        assert!(known_contract.is_none());
    }
}