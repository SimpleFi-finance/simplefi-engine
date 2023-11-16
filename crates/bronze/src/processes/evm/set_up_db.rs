use chains_types::get_chain;

#[tokio::main]
async fn main() {
    let chain_id = "1"; //TODO: switch to env
    // let data_level = SupportedDataLevels::from_str("bronze").unwrap();

    match chain_id {
        "1" => {
            // let db = SupportedChains::EthereumMainnet.get_db();

            // let db = Mongo::new(&db).await.unwrap();
            let chain = get_chain(chain_id).unwrap();
            // let block_coll =
            //     &chain.resolve_collection_name(&SupportedDataTypes::Blocks, &data_level);
            // let log_coll = &chain.resolve_collection_name(&SupportedDataTypes::Logs, &data_level);
            // let txs_coll =
            //     &chain.resolve_collection_name(&SupportedDataTypes::Transactions, &data_level);
            // let decoding_error_coll =
            //     &chain.resolve_collection_name(&SupportedDataTypes::DecodingError, &data_level);

            // let (_, _, _, _) = tokio::join!(
            //     init_blocks_bronze::<Block>(&db, block_coll),
            //     init_logs_bronze::<Log>(&db, log_coll),
            //     init_txs_bronze::<Tx>(&db, txs_coll),
            //     init_decoding_error_bronze::<DecodingError>(&db, decoding_error_coll),
            // );
        }
        _ => panic!("Chain not implemented to subscribe to blocks"),
    };
}
