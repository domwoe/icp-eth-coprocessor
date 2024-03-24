use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::{self, api::call::CallResult};

pub const CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai

#[derive(CandidType, Deserialize)]
pub enum Auth {
    RegisterProvider,
    FreeRpc,
    PriorityRpc,
    Manage,
}

#[derive(CandidType, Deserialize)]
pub enum EthSepoliaService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Deserialize)]
pub enum EthMainnetService {
    Alchemy,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>),
    Custom { chainId: u64, services: Vec<RpcApi> },
    EthMainnet(Option<Vec<EthMainnetService>>),
}

#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(u128),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistoryArgs {
    pub blockCount: u128,
    pub newestBlock: BlockTag,
    pub rewardPercentiles: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistory {
    pub reward: Vec<Vec<u128>>,
    pub gasUsedRatio: Vec<f64>,
    pub oldestBlock: u128,
    pub baseFeePerGas: Vec<u128>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ProviderError {
    TooFewCycles { expected: u128, received: u128 },
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ValidationError {
    CredentialPathNotAllowed,
    HostNotAllowed(String),
    CredentialHeaderNotAllowed,
    UrlParseError(String),
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize)]
pub enum FeeHistoryResult {
    Ok(Option<FeeHistory>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RpcService {
    EthSepolia(EthSepoliaService),
    Custom(RpcApi),
    EthMainnet(EthMainnetService),
    Chain(u64),
    Provider(u64),
}

#[derive(CandidType, Deserialize)]
pub enum MultiFeeHistoryResult {
    Consistent(FeeHistoryResult),
    Inconsistent(Vec<(RpcService, FeeHistoryResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct Block {
    pub miner: String,
    pub totalDifficulty: u128,
    pub receiptsRoot: String,
    pub stateRoot: String,
    pub hash: String,
    pub difficulty: u128,
    pub size: u128,
    pub uncles: Vec<String>,
    pub baseFeePerGas: u128,
    pub extraData: String,
    pub transactionsRoot: Option<String>,
    pub sha3Uncles: String,
    pub nonce: u128,
    pub number: u128,
    pub timestamp: u128,
    pub transactions: Vec<String>,
    pub gasLimit: u128,
    pub logsBloom: String,
    pub parentHash: String,
    pub gasUsed: u128,
    pub mixHash: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlockByNumberResult {
    Ok(Block),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBlockByNumberResult {
    Consistent(GetBlockByNumberResult),
    Inconsistent(Vec<(RpcService, GetBlockByNumberResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetLogsArgs {
    pub fromBlock: Option<BlockTag>,
    pub toBlock: Option<BlockTag>,
    pub addresses: Vec<String>,
    pub topics: Option<Vec<Vec<String>>>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct LogEntry {
    pub transactionHash: Option<String>,
    pub blockNumber: Option<u128>,
    pub data: String,
    pub blockHash: Option<String>,
    pub transactionIndex: Option<u128>,
    pub topics: Vec<String>,
    pub address: String,
    pub logIndex: Option<u128>,
    pub removed: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum GetLogsResult {
    Ok(Vec<LogEntry>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetLogsResult {
    Consistent(GetLogsResult),
    Inconsistent(Vec<(RpcService, GetLogsResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionCountResult {
    Ok(u128),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionCountResult {
    Consistent(GetTransactionCountResult),
    Inconsistent(Vec<(RpcService, GetTransactionCountResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct TransactionReceipt {
    pub to: String,
    pub status: u128,
    pub transactionHash: String,
    pub blockNumber: u128,
    pub from: String,
    pub logs: Vec<LogEntry>,
    pub blockHash: String,
    pub r#type: String,
    pub transactionIndex: u128,
    pub effectiveGasPrice: u128,
    pub logsBloom: String,
    pub contractAddress: Option<String>,
    pub gasUsed: u128,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionReceiptResult {
    Ok(Option<TransactionReceipt>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionReceiptResult {
    Consistent(GetTransactionReceiptResult),
    Inconsistent(Vec<(RpcService, GetTransactionReceiptResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionStatus {
    Ok,
    NonceTooLow,
    NonceTooHigh,
    InsufficientFunds,
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionResult {
    Ok(SendRawTransactionStatus),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct Metrics {
    pub cyclesWithdrawn: u128,
    pub responses: Vec<((String, String, String), u64)>,
    pub errNoPermission: u64,
    pub inconsistentResponses: Vec<((String, String), u64)>,
    pub cyclesCharged: Vec<((String, String), u128)>,
    pub requests: Vec<((String, String), u64)>,
    pub errHttpOutcall: Vec<((String, String), u64)>,
    pub errHostNotAllowed: Vec<(String, u64)>,
}

#[derive(CandidType, Deserialize)]
pub struct ProviderView {
    pub cyclesPerCall: u64,
    pub owner: Principal,
    pub hostname: String,
    pub primary: bool,
    pub chainId: u64,
    pub cyclesPerMessageByte: u64,
    pub providerId: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ManageProviderArgs {
    pub service: Option<RpcService>,
    pub primary: Option<bool>,
    pub providerId: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterProviderArgs {
    pub cyclesPerCall: u64,
    pub credentialPath: String,
    pub hostname: String,
    pub credentialHeaders: Option<Vec<HttpHeader>>,
    pub chainId: u64,
    pub cyclesPerMessageByte: u64,
}

#[derive(CandidType, Deserialize)]
pub enum RequestResult {
    Ok(String),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RequestCostResult {
    Ok(u128),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProviderArgs {
    pub cyclesPerCall: Option<u64>,
    pub credentialPath: Option<String>,
    pub hostname: Option<String>,
    pub credentialHeaders: Option<Vec<HttpHeader>>,
    pub primary: Option<bool>,
    pub cyclesPerMessageByte: Option<u64>,
    pub providerId: u64,
}

pub struct EvmRpcCanister;
impl EvmRpcCanister {
    pub async fn get_logs(
        services: RpcServices,
        config: Option<RpcConfig>,
        args: GetLogsArgs,
        cycles: u128,
    ) -> CallResult<(MultiGetLogsResult,)> {
        ic_cdk::api::call::call_with_payment128(
            CANISTER_ID,
            "eth_getLogs",
            (services, config, args),
            cycles,
        )
        .await
    }

    pub async fn eth_fee_history(
        services: RpcServices,
        config: Option<RpcConfig>,
        args: FeeHistoryArgs,
        cycles: u128,
    ) -> CallResult<(MultiFeeHistoryResult,)> {
        ic_cdk::api::call::call_with_payment128(
            CANISTER_ID,
            "eth_feeHistory",
            (services, config, args),
            cycles,
        )
        .await
    }

    pub async fn eth_send_raw_transaction(
        services: RpcServices,
        config: Option<RpcConfig>,
        raw_tx: String,
        cycles: u128,
    ) -> CallResult<(MultiSendRawTransactionResult,)> {
        ic_cdk::api::call::call_with_payment128(
            CANISTER_ID,
            "eth_sendRawTransaction",
            (services, config, raw_tx),
            cycles,
        )
        .await
    }
}

//TODO: FIX inconsistency with topic type
pub async fn get_logs(
    network: String,
    addresses: Vec<String>,
    topics: Option<Vec<Vec<String>>>,
    from_block: u128,
    to_block: BlockTag,
) -> Vec<LogEntry> {
    ic_cdk::print(format!(
        "Getting logs from block {} to block {:?}",
        from_block, to_block
    ));

    let config = None;
    let args = GetLogsArgs {
        addresses,
        fromBlock: Some(BlockTag::Number(from_block)),
        toBlock: Some(to_block),
        topics: None,
    };

    let services = match network.as_str() {
        "EthSepolia" => RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy])),
        "EthMainnet" => RpcServices::EthMainnet(None),
        _ => RpcServices::EthSepolia(None),
    };

    let cycles = 10000000;
    match EvmRpcCanister::get_logs(services, config, args, cycles).await {
        Ok((res,)) => match res {
            MultiGetLogsResult::Consistent(logs) => match logs {
                GetLogsResult::Ok(logs) => logs,
                GetLogsResult::Err(e) => {
                    ic_cdk::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiGetLogsResult::Inconsistent(_) => {
                ic_cdk::trap("Logs are inconsistent");
            }
        },
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}

pub async fn fee_history(
    network: String,
    block_count: u128,
    newest_block: BlockTag,
    reward_percentiles: Option<serde_bytes::ByteBuf>,
) -> FeeHistory {
    let config = None;
    let args = FeeHistoryArgs {
        blockCount: block_count,
        newestBlock: newest_block,
        rewardPercentiles: reward_percentiles,
    };

    let services = match network.as_str() {
        "EthSepolia" => RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy])),
        "EthMainnet" => RpcServices::EthMainnet(None),
        _ => RpcServices::EthSepolia(None),
    };

    let cycles = 10000000;
    match EvmRpcCanister::eth_fee_history(services, config, args, cycles).await {
        Ok((res,)) => match res {
            MultiFeeHistoryResult::Consistent(fee_history) => match fee_history {
                FeeHistoryResult::Ok(fee_history) => fee_history.unwrap(),
                FeeHistoryResult::Err(e) => {
                    ic_cdk::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiFeeHistoryResult::Inconsistent(_) => {
                ic_cdk::trap("Fee history is inconsistent");
            }
        },
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}

pub async fn send_raw_transaction(network: String, raw_tx: String) -> SendRawTransactionStatus {
    let config = None;
    let services = match network.as_str() {
        "EthSepolia" => RpcServices::EthSepolia(Some(vec![EthSepoliaService::Alchemy])),
        "EthMainnet" => RpcServices::EthMainnet(None),
        _ => RpcServices::EthSepolia(None),
    };

    let cycles = 10000000;
    match EvmRpcCanister::eth_send_raw_transaction(services, config, raw_tx, cycles).await {
        Ok((res,)) => match res {
            MultiSendRawTransactionResult::Consistent(status) => match status {
                SendRawTransactionResult::Ok(status) => status,
                SendRawTransactionResult::Err(e) => {
                    ic_cdk::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiSendRawTransactionResult::Inconsistent(_) => {
                ic_cdk::trap("Status is inconsistent");
            }
        },
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}
