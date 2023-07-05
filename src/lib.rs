use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeSeq;
use reqwest::{Client, Error};


const API_URL: &'static str = "https://api.etherscan.io/api";
const GOERLI_API_URL: &'static str = "https://api-goerli.etherscan.io/api";
const SEPOLIA_API_URL: &'static str = "https://api-sepolia.etherscan.io/api";

// region: API Request structs

#[derive(Debug, Serialize)]
struct BaseApiRequest<'a> {
    module: &'a str,
    action: &'a str,
    apikey: &'a str,
}

// region: Acounts and Transactions endpoints

#[derive(Debug, Serialize)]
struct AddressTagQuery<'a> {
    address: &'a str,
    tag: &'a str,
}

#[derive(Debug, Serialize)]
struct TxListPaginatedQuery<'a> {
    address: &'a str,
    startblock: i64,
    endblock: i64,
    page: i64,
    offset: i64,
    sort: &'a str,
}

#[derive(Debug, Serialize)]
struct TxHashQuery<'a> {
    txhash: &'a str,
}

#[derive(Debug, Serialize)]
struct BlockRangePaginatedQuery<'a> {
    startblock: i64,
    endblock: i64,
    page: i64,
    offset: i64,
    sort: &'a str,
}

#[derive(Debug, Serialize)]
struct TokenEventsPaginatedQuery<'a> {
    address: &'a str,
    contractaddress: &'a str,
    page: i64,
    offset: i64,
    startblock: i64,
    endblock: i64,
    sort: &'a str,  // "asc" or "desc"
}

#[derive(Debug, Serialize)]
struct AddressBlocktypePaginatedQuery<'a> {
    address: &'a str,
    blocktype: &'a str,  // "blocks" or "uncles"
    page: i64,
    offset: i64,
    sort: &'a str,
}

#[derive(Debug, Serialize)]
struct ContractByAddressBlockRangePaginatedQuery<'a> {
    address: &'a str,
    contractaddress: &'a str,
    page: i64,
    offset: i64,
    startblock: i64,
    endblock: i64,
    sort: &'a str,
}

#[derive(Debug, Serialize)]
struct AddressBlockNumberQuery<'a> {
    address: &'a str,
    blockno: i64,  // block number to query
}

// endregion

// region: Contracts endpoints

#[derive(Debug, Serialize)]
struct AddressQuery<'a> {
    address: &'a str,
}

#[derive(Debug, Serialize)]
struct ContractAddressQuery<'a> {
    contractaddress: &'a str,
}

// TODO: Verify contract source code endpoint
// TODO: Verify proxy contract endpoint 

// endregion

// region: Blocks endpoints

#[derive(Debug, Serialize)]
struct BlockNumberQuery {
    blockno: i64,
}

#[derive(Debug, Serialize)]
struct BlockTimestampQuery<'a> {
    timestamp: i64,    // Unix timestamp in seconds
    closest: &'a str,  // "before" or "after"
}

#[derive(Debug, Serialize)]
struct DateRangeQuery<'a> {
    startdate: &'a str,  // yyyy-MM-dd format, eg. 2020-02-28
    enddate: &'a str,    // yyyy-MM-dd format, eg. 2020-02-28
    sort: &'a str,       // "asc" or "desc"
}

// endregion

// region: Logs endpoints

#[derive(Debug, Serialize)]
struct EventLogAddressPaginatedQuery<'a> {
    address: &'a str,
    fromblock: i64,
    toblock: i64,
    page: i64,
    offset: i64,
}

#[derive(Debug)]
struct EventLogTopicPaginatedQuery<'a> {
    fromblock: i64,
    toblock: i64,
    topic0: &'a str,
    topic1: &'a str,  // optional, requires topic0
    topic2: &'a str,  // optional, requires topic1
    topic3: &'a str,  // optional, requires topic2
    topic0_1_opr: &'a str,  // "and" or "or"
    topic1_2_opr: &'a str,  // "and" or "or"
    topic2_3_opr: &'a str,  // "and" or "or"
    page: i64,
    offset: i64,
}

impl Serialize for EventLogTopicPaginatedQuery<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer {
        // Do not serialize the field if it is empty
        let mut seq: <S as Serializer>::SerializeSeq = serializer.serialize_seq(Some(10))?;
        seq.serialize_element(&self.fromblock)?;
        seq.serialize_element(&self.toblock)?;
        seq.serialize_element(&self.topic0)?;
        if self.topic1 != "" {
            seq.serialize_element(&self.topic1)?;
            seq.serialize_element(&self.topic0_1_opr)?;
        }
        if self.topic2 != "" {
            seq.serialize_element(&self.topic2)?;
            seq.serialize_element(&self.topic1_2_opr)?;
        }
        if self.topic3 != "" {
            seq.serialize_element(&self.topic3)?;
            seq.serialize_element(&self.topic2_3_opr)?;
        }
        seq.serialize_element(&self.page)?;
        seq.serialize_element(&self.offset)?;
        seq.end()
    }
}

// endregion

// region: Geth/Parity proxy endpoints

#[derive(Debug, Serialize)]
struct BlockNumberBoolQuery<'a> {
    tag: &'a str,  // block number in hex
    boolean: bool,
}

#[derive(Debug, Serialize)]
struct BlockNumberIndexQuery<'a> {
    tag: &'a str,    // block number in hex
    index: &'a str,  // position of uncle in block in hex
}

#[derive(Debug, Serialize)]
struct BlockNumberHexQuery<'a> {
    tag: &'a str,  // block number in hex
}

#[derive(Debug, Serialize)]
struct RawTxQuery<'a> {
    hex: &'a str,  // raw transaction in hex
}

#[derive(Debug, Serialize)]
struct CallQuery<'a> {
    to: &'a str,    // contract address in hex
    data: &'a str,  // hash of the method signature and encoded parameters in hex
    tag: &'a str,   // earlist, latest, or pending
}

#[derive(Debug, Serialize)]
struct StoragePositionQuery<'a> {
    address: &'a str,  // address to get storage position from
    position: &'a str,  // storage position in hex
    tag: &'a str,    // earlist, latest, or pending
}

#[derive(Debug, Serialize)]
struct EstimateGasQuery<'a> {
    data: &'a str,  // hash of the method signature and encoded parameters in hex
    to: &'a str,    // contract address in hex
    value: &'a str, // value to send in hex
    gas: &'a str,   // amount of gas provided for the transaction in hex
    gasPrice: &'a str,  // gas price paid for each unit of gas, in wei post EIP-1559, in hex
}

// endregion

// region: Tokens endpoints

#[derive(Debug, Serialize)]
struct ContractAddressesQuery<'a> {
    contractaddresses: &'a str,
}

#[derive(Debug, Serialize)]
struct ContractByAddressQuery<'a> {
    address: &'a str,          // address to check
    contractaddress: &'a str,  // contract address of token
}

#[derive(Debug, Serialize)]
struct ContractByAddressPaginatedQuery<'a> {
    address: &'a str,          // address to check
    contractaddress: &'a str,  // contract address of token
    page: i64,
    offset: i64,
}

#[derive(Debug, Serialize)]
struct ContractByBlockNumberQuery<'a> {
    contractaddress: &'a str,  // contract address of token
    blockno: i64,              // block number to query
}

#[derive(Debug, Serialize)]
struct ContractByAddressBlockNumberQuery<'a> {
    address: &'a str,          // address to check
    contractaddress: &'a str,  // contract address of token
    blockno: i64,              // block number to query
}

#[derive(Debug, Serialize)]
struct ContractAddressPaginatedQuery<'a> {
    contractaddress: &'a str,
    page: i64,
    offset: i64,
}

#[derive(Debug, Serialize)]
struct AddressPaginatedQuery<'a> {
    address: &'a str,
    page: i64,
    offset: i64,
}

// endregion

// region: Gas Tracker endpoints

#[derive(Debug, Serialize)]
struct GasPriceQuery<'a> {
    gasprice: &'a str,
}

// endregion

// region: Stats endpoints

#[derive(Debug, Serialize)]
struct BlockchainSizeQuery<'a> {
    startdate: &'a str,   // date in format yyyy-mm-dd, 2020-02-28
    enddate: &'a str,     // date in format yyyy-mm-dd, 2020-02-28
    clienttype: &'a str,  // geth or parity
    syncmode: &'a str,    // default or archive
    sort: &'a str,        // asc or desc
}

// endregion


#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    result: String,
}

// Create a struct to hold the API key and the HTTP client
struct AsyncClient {
    api_key: String,
    client: Client,
}

// Init
impl AsyncClient {
    // Method to create a new instance of EtherscanClient with the API key
    fn new(api_key: &str) -> Self {
        AsyncClient {
            api_key: api_key.to_owned(),
            client: Client::new(),
        }
    }

    async fn get(&self, module: &str, action: &str, params: impl Serialize) -> Result<String, Error> {
        // Create the JSON-RPC request
        let base_request: BaseApiRequest = BaseApiRequest {
            module,
            action,
            apikey: &self.api_key,
        };

        // Send the request and await the response
        let response: ApiResponse = self.client
            .get(API_URL)
            .query(&base_request)
            .query(&params)
            .send()
            .await?
            .json::<ApiResponse>()
            .await?;

        // Extract the balance from the response and return it
        Ok(response.result.to_string())
    }

}

// Accounts API
// TODO: Make this a trait
impl AsyncClient {
    async fn get_balance(&self, address: &str) -> Result<String, Error> {
        let params: AddressTagQuery = AddressTagQuery {
            address,
            tag: "latest",
        };
        self.get("account", "balance", params).await
    }

    async fn get_balance_multi(&self, addresses: &Vec<&str>) -> Result<String, Error> {
        let addresses: String = addresses.join(",");
        let params: AddressTagQuery = AddressTagQuery {
            address: &addresses,
            tag: "latest",
        };
        self.get("account", "balancemulti", params).await
    }

    async fn get_tx_list(&self, address: &str, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: TxListPaginatedQuery<'_> = TxListPaginatedQuery {
            address,
            startblock: start_block,
            endblock: end_block,
            page,
            offset,
            sort,
        };
        self.get("account", "txlist", params).await
    }

    async fn get_tx_list_internal(&self, address: &str, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: TxListPaginatedQuery<'_> = TxListPaginatedQuery {
            address,
            startblock: start_block,
            endblock: end_block,
            page,
            offset,
            sort,
        };
        self.get("account", "txlistinternal", params).await
    }

    async fn get_tx_list_internal_by_hash(&self, tx_hash: &str) -> Result<String, Error> {
        let params: TxHashQuery<'_> = TxHashQuery {
            txhash: tx_hash,
        };
        self.get("account", "txlistinternal", params).await
    }

    async fn get_tx_list_internal_by_blockrange(&self, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: BlockRangePaginatedQuery<'_> = BlockRangePaginatedQuery {
            startblock: start_block,
            endblock: end_block,
            page,
            offset,
            sort,
        };
        self.get("account", "txlistinternal", params).await
    }

    async fn get_erc20_transfer_events(&self, address: &str, contract_address: &str, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: TokenEventsPaginatedQuery<'_> = TokenEventsPaginatedQuery {
            address,
            contractaddress: contract_address,
            startblock: start_block,
            endblock: end_block,
            page,
            offset,
            sort
        };
        self.get("account", "tokentx", params).await
    }

    async fn get_erc721_transfer_events(&self, address: &str, contract_address: &str, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: TokenEventsPaginatedQuery<'_> = TokenEventsPaginatedQuery {
            address,
            contractaddress: contract_address,
            startblock: start_block,
            endblock: end_block,
            page,
            offset,
            sort
        };
        self.get("account", "tokennfttx", params).await
    }

    async fn get_erc1155_transfer_events(&self, address: &str, contract_address: &str, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: TokenEventsPaginatedQuery<'_> = TokenEventsPaginatedQuery {
            address,
            contractaddress: contract_address,
            startblock: start_block,
            endblock: end_block,
            page,
            offset,
            sort
        };
        self.get("account", "tokennfttx", params).await
    }

    async fn get_mined_blocks(&self, address: &str, blocktype: &str, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: AddressBlocktypePaginatedQuery<'_> = AddressBlocktypePaginatedQuery {
            address,
            blocktype,
            page,
            offset,
            sort
        };
        self.get("account", "getminedblocks", params).await
    }

    async fn get_tx_list_beacon_withdrawal(&self, address: &str, contract_address: &str, start_block: i64, end_block: i64, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: ContractByAddressBlockRangePaginatedQuery<'_> = ContractByAddressBlockRangePaginatedQuery {
            address,
            contractaddress: contract_address,
            page,
            offset,
            startblock: start_block,
            endblock: end_block,
            sort,
        };
        self.get("account", "txlistbeacon", params).await
    }

    async fn get_balance_history(&self, address: &str, blockno: i64) -> Result<String, Error> {
        let params: AddressBlockNumberQuery<'_> = AddressBlockNumberQuery {
            address,
            blockno
        };
        self.get("account", "balancehistory", params).await
    }
}

// Contract API
// TODO: Make this a trait
impl AsyncClient {
    async fn contract_abi(&self, address: &str) -> Result<String, Error> {
        let params: AddressQuery<'_> = AddressQuery {
            address,
        };
        self.get("contract", "getabi", params).await
    }

    async fn contract_source_code(&self, address: &str) -> Result<String, Error> {
        let params: AddressQuery<'_> = AddressQuery {
            address,
        };
        self.get("contract", "getsourcecode", params).await
    }

    async fn contract_creation(&self, contract_addresses: &Vec<&str>) -> Result<String, Error> {
        let contract_addresses: String = contract_addresses.join(",");
        let params: ContractAddressesQuery<'_> = ContractAddressesQuery {
            contractaddresses: &contract_addresses,
        };
        self.get("contract", "getsourcecode", params).await
    }
}

// Transaction API
impl AsyncClient {
    async fn transaction_status(&self, tx_hash: &str) -> Result<String, Error> {
        let params: TxHashQuery<'_> = TxHashQuery {
            txhash: tx_hash,
        };
        self.get("transaction", "getstatus", params).await
    }

    async fn transaction_receipt_status(&self, tx_hash: &str) -> Result<String, Error> {
        let params: TxHashQuery<'_> = TxHashQuery {
            txhash: tx_hash,
        };
        self.get("transaction", "gettxreceiptstatus", params).await
    }
}

// Block API
impl AsyncClient {
    async fn block_reward(&self, blockno: i64) -> Result<String, Error> {
        let params: BlockNumberQuery = BlockNumberQuery {
            blockno
        };
        self.get("block", "getblockreward", params).await
    }

    async fn block_countdown(&self, blockno: i64) -> Result<String, Error> {
        let params: BlockNumberQuery = BlockNumberQuery {
            blockno
        };
        self.get("block", "getblockcountdown", params).await
    }
    
    async fn block_number_by_timestamp(&self, timestamp: i64, closest: &str) -> Result<String, Error> {
        let params: BlockTimestampQuery<'_> = BlockTimestampQuery {
            timestamp,
            closest
        };
        self.get("block", "getblocknobytime", params).await
    }

    async fn daily_average_blocksize(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort
        };
        self.get("block", "getdailyavgblocksize", params).await
    }

    async fn daily_block_count(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort
        };
        self.get("block", "getdailyblockcount", params).await
    }

    async fn daily_block_rewards(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort
        };
        self.get("block", "getdailyblockrewards", params).await
    }

    async fn daily_block_time(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort
        };
        self.get("block", "getdailyblocktime", params).await
    }

    async fn daily_uncle_block_count(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort
        };
        self.get("block", "getdailyuncleblockcount", params).await
    }


}

// Logs API
impl AsyncClient {
    async fn logs_by_address(&self, address: &str, from_block: i64, to_block: i64, page: i64, offset: i64) -> Result<String, Error> {
        let params: EventLogAddressPaginatedQuery<'_> = EventLogAddressPaginatedQuery {
            address,
            fromblock: from_block,
            toblock: to_block,
            page,
            offset
        };
        self.get("logs", "getLogs", params).await
    }

    // async fn logs_by_topic(&self, topic0: &str, topic1: &str, topic2: &str, topic3: &str, from_block: i64, to_block: i64, page: i64, offset: i64) -> Result<String, Error> {
    //     let params: EventLogTopicPaginatedQuery<'_> = EventLogTopicPaginatedQuery {
    //         topic0,
    //         topic1,
    //         topic2,
    //         topic3,
    //         fromblock: from_block,
    //         toblock: to_block,
    //         page,
    //         offset
    //     };
    //     self.get("logs", "getLogs", params).await
    // }

    // async fn logs_by_address_topic(&self, address: &str, topic0: &str, topic1: &str, topic2: &str, topic3: &str, from_block: i64, to_block: i64, page: i64, offset: i64) -> Result<String, Error> {
    //     let params: EventLogAddressTopicPaginatedQuery<'_> = EventLogAddressTopicPaginatedQuery {
    //         address,
    //         topic0,
    //         topic1,
    //         topic2,
    //         topic3,
    //         fromblock: from_block,
    //         toblock: to_block,
    //         page,
    //         offset
    //     };
    //     self.get("logs", "getLogs", params).await
    // }

}

// Geth/Parity Proxy API
impl AsyncClient {
    async fn eth_get_block_number(&self) -> Result<String, Error> {
        self.get("proxy", "eth_blockNumber", ()).await
    }

    async fn eth_get_block_by_number(&self, blockno: i64, show_full_tx: bool) -> Result<String, Error> {
        let params: BlockNumberBoolQuery = BlockNumberBoolQuery {
            tag: &format!("0x{:x}", blockno),
            boolean: show_full_tx
        };
        self.get("proxy", "eth_getBlockByNumber", params).await
    }

    async fn eth_get_uncle_by_block_number_and_index(&self, blockno: i64, index: i64) -> Result<String, Error> {
        let params: BlockNumberIndexQuery = BlockNumberIndexQuery {
            tag: &format!("0x{:x}", blockno),
            index: &format!("0x{:x}", index),
        };
        self.get("proxy", "eth_getUncleByBlockNumberAndIndex", params).await
    }

    async fn eth_get_transaction_by_hash(&self, tx_hash: &str) -> Result<String, Error> {
        let params: TxHashQuery<'_> = TxHashQuery {
            txhash: tx_hash
        };
        self.get("proxy", "eth_getTransactionByHash", params).await
    }

    async fn eth_get_transaction_by_block_number_and_index(&self, blockno: i64, index: i64) -> Result<String, Error> {
        let params: BlockNumberIndexQuery = BlockNumberIndexQuery {
            tag: &format!("0x{:x}", blockno),
            index: &format!("0x{:x}", index),
        };
        self.get("proxy", "eth_getTransactionByBlockNumberAndIndex", params).await
    }

    async fn eth_get_transaction_count(&self, address: &str, tag: &str) -> Result<String, Error> {
        let params: AddressTagQuery<'_> = AddressTagQuery {
            address,
            tag,
        };
        self.get("proxy", "eth_getTransactionCount", params).await
    }
    
    async fn eth_send_raw_transaction(&self, hex: &str) -> Result<String, Error> {
        let params: RawTxQuery<'_> = RawTxQuery {
            hex
        };
        self.get("proxy", "eth_sendRawTransaction", params).await
    }

    async fn eth_get_transaction_receipt(&self, tx_hash: &str) -> Result<String, Error> {
        let params: TxHashQuery<'_> = TxHashQuery {
            txhash: tx_hash
        };
        self.get("proxy", "eth_getTransactionReceipt", params).await
    }

    async fn eth_call(&self, to: &str, data: &str, tag: &str) -> Result<String, Error> {
        let params: CallQuery<'_> = CallQuery {
            to,
            data,
            tag,
        };
        self.get("proxy", "eth_call", params).await
    }

    async fn eth_get_code(&self, address: &str, tag: &str) -> Result<String, Error> {
        let params: AddressTagQuery<'_> = AddressTagQuery {
            address,
            tag,
        };
        self.get("proxy", "eth_getCode", params).await
    }

    async fn eth_get_storage_at(&self, address: &str, position: &str, tag: &str) -> Result<String, Error> {
        let params: StoragePositionQuery<'_> = StoragePositionQuery {
            address,
            position,
            tag,
        };
        self.get("proxy", "eth_getStorageAt", params).await
    }

    async fn eth_gas_price(&self) -> Result<String, Error> {
        self.get("proxy", "eth_gasPrice", ()).await
    }

    async fn eth_estimate_gas(&self, to: &str, data: &str, value: i64, gas: i64, gas_price: i64) -> Result<String, Error> {
        let params: EstimateGasQuery<'_> = EstimateGasQuery {
            to,
            data,
            value: &format!("0x{:x}", value),
            gas: &format!("0x{:x}", gas),
            gasPrice: &format!("0x{:x}", gas_price),
        };
        self.get("proxy", "eth_estimateGas", params).await
    }
}

// Tokens API
impl AsyncClient {
    async fn token_total_supply(&self, contract_address: &str) -> Result<String, Error> {
        let params: ContractAddressQuery<'_> = ContractAddressQuery {
            contractaddress: contract_address,
        };
        self.get("tokens", "tokenSupply", params).await
    }

    async fn token_balance(&self, contract_address: &str, address: &str, tag: &str) -> Result<String, Error> {
        let params: ContractByAddressQuery<'_> = ContractByAddressQuery {
            contractaddress: contract_address,
            address,
        };
        self.get("tokens", "tokenBalance", params).await
    }

    async fn token_supply_history(&self, contract_address: &str, blockno: i64, offset: i64, page: i64, sort: &str) -> Result<String, Error> {
        let params: ContractByBlockNumberQuery<'_> = ContractByBlockNumberQuery {
            contractaddress: contract_address,
            blockno: blockno,
        };
        self.get("tokens", "tokenSupplyHistory", params).await
    }

    async fn token_balance_history(&self, contract_address: &str, address: &str, blockno: i64, offset: i64, page: i64, sort: &str) -> Result<String, Error> {
        let params: ContractByAddressBlockNumberQuery<'_> = ContractByAddressBlockNumberQuery {
            contractaddress: contract_address,
            address,
            blockno: blockno,
        };
        self.get("tokens", "tokenBalanceHistory", params).await
    }

    async fn token_holder_list(&self, contract_address: &str, page: i64, offset: i64, sort: &str) -> Result<String, Error> {
        let params: ContractAddressPaginatedQuery<'_> = ContractAddressPaginatedQuery {
            contractaddress: contract_address,
            page: page,
            offset: offset,
        };
        self.get("tokens", "tokennholderlist", params).await
    }

    async fn token_info(&self, contract_address: &str) -> Result<String, Error> {
        let params: ContractAddressQuery<'_> = ContractAddressQuery {
            contractaddress: contract_address,
        };
        self.get("tokens", "tokenInfo", params).await
    }

    async fn erc20_token_balance(&self, address: &str, page: i64, offset: i64) -> Result<String, Error> {
        let params: AddressPaginatedQuery<'_> = AddressPaginatedQuery {
            address,
            page,
            offset,
        };
        self.get("tokens", "tokenBalance", params).await
    }

    async fn erc721_token_inventory(&self, address: &str, page: i64, offset: i64) -> Result<String, Error> {
        let params: AddressPaginatedQuery<'_> = AddressPaginatedQuery {
            address,
            page,
            offset,
        };
        self.get("tokens", "tokennfttx", params).await
    }

    async fn erc721_token_inventory_by_contract(&self, contract_address: &str, address: &str, page: i64, offset: i64) -> Result<String, Error> {
        let params: ContractByAddressPaginatedQuery<'_> = ContractByAddressPaginatedQuery {
            contractaddress: contract_address,
            address,
            page,
            offset,
        };
        self.get("tokens", "tokennfttx", params).await
    }

}

// Gas Tracker API
impl AsyncClient {
    async fn estimate_confirmation_time(&self, gas_price: i64) -> Result<String, Error> {
        let params: GasPriceQuery<'_> = GasPriceQuery {
            gasprice: &format!("0x{:x}", gas_price),
        };
        self.get("gastracker", "gasestimate", params).await
    }

    async fn gas_oracle(&self) -> Result<String, Error> {
        self.get("gastracker", "gasoracle", ()).await
    }

    async fn daily_average_gas_limit(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailyavggaslimit", params).await
    }

    async fn daily_total_gas_used(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailygasused", params).await
    }

    async fn daily_average_gas_price(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailyavggasprice", params).await
    }
}

// Stats API
impl AsyncClient {
    async fn total_eth_supply(&self) -> Result<String, Error> {
        self.get("stats", "ethsupply", ()).await
    }

    async fn total_eth2_supply(&self) -> Result<String, Error> {
        self.get("stats", "ethsupply2", ()).await
    }

    async fn eth_price(&self) -> Result<String, Error> {
        self.get("stats", "ethprice", ()).await
    }

    async fn chain_size(&self, start_date: &str, end_date: &str, client_type: &str, sync_mode: &str, sort: &str) -> Result<String, Error> {
        let params: BlockchainSizeQuery<'_> = BlockchainSizeQuery {
            startdate: start_date,
            enddate: end_date,
            clienttype: client_type,
            syncmode: sync_mode,
            sort,
        };
        self.get("stats", "chainsize", params).await
    }

    async fn total_node_count(&self) -> Result<String, Error> {
        self.get("stats", "nodecount", ()).await
    }

    async fn daily_total_transaction_fee(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailytxnsfee", params).await
    }

    async fn daily_new_address_count(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "newaddress", params).await
    }

    async fn daily_network_utilization(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "ethusd", params).await
    }

    async fn daily_average_hash_rate(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailyhashrate", params).await
    }

    async fn dailt_transaction_count(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailytxns", params).await
    }

    async fn daily_average_difficulty(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "dailyavgdifficulty", params).await
    }

    async fn daily_market_cap_history(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "ethdailymarketcap", params).await
    }

    async fn daily_eth_price_history(&self, start_date: &str, end_date: &str, sort: &str) -> Result<String, Error> {
        let params: DateRangeQuery<'_> = DateRangeQuery {
            startdate: start_date,
            enddate: end_date,
            sort,
        };
        self.get("stats", "ethdailyprice", params).await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_balance() {
        // Create a new instance of the EtherscanClient
        let client: AsyncClient = AsyncClient::new("YourApiKeyToken");

        // Make the API call
        let balance: String = client.get_balance("0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae").await.unwrap();

        // Print the balance
        println!("Balance: {}", balance);
    }

}
