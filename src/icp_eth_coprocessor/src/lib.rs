use crate::evm_rpc::BlockTag;

use ethers_core::types::U256;
use ethers_core::utils::keccak256;

use evm_rpc::SendRawTransactionStatus;

use ic_cdk_timers::set_timer_interval;

use std::time::Duration;

mod evm_rpc;
mod evm_signer;
mod memory;

use memory::{CONFIG, STATE};

#[ic_cdk::init]
fn init() {
    // Get tECDSA public key, calculate EVM address and store them in the state
    ic_cdk_timers::set_timer(Duration::ZERO, || {
        ic_cdk::spawn(async {
            let pubkey = evm_signer::get_public_key().await;

            let evm_address = evm_signer::pubkey_bytes_to_address(pubkey.as_slice());

            STATE.with(|state| {
                let mut s = state.borrow_mut().get().clone();
                s.ecdsa_pub_key = Some(pubkey);
                s.evm_address = Some(evm_address);
                state.borrow_mut().set(s).expect("Failed to set state");
            });
        });
    });

    set_timer_interval(Duration::from_secs(60), || ic_cdk::spawn(sync_logs()));
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    set_timer_interval(Duration::from_secs(60), || ic_cdk::spawn(sync_logs()));
}

#[ic_cdk::update]
fn set_contract(contract: String) {
    CONFIG.with(|config| {
        let mut c = config.borrow().get().clone();
        c.evm_contract = Some(contract);
        let _ = config.borrow_mut().set(c);
    });
}

#[ic_cdk::query]
fn get_evm_address() -> String {
    let address = STATE.with(|state| {
        let s = state.borrow();
        let s = s.get();
        s.evm_address.clone()
    });
    //TODO: return error if address is None
    address.unwrap_or("Not initialized".to_string())
}

async fn sync_logs() {
    let config = CONFIG.with(|config| config.borrow().get().clone());
    let state = STATE.with(|state| state.borrow().get().clone());

    if config.evm_contract.is_none() {
        return;
    } else {
        let logs = evm_rpc::get_logs(
            config.evm_network.clone(),
            [config.evm_contract.clone().unwrap()].to_vec(),
            None,
            state.evm_block_height+1,
            BlockTag::Latest,
        )
        .await;

        for (index, event) in logs.iter().enumerate() {
            let job_id = hex_to_u64(&event.data).unwrap_or(0);
            process_job(job_id).await;

            // Update the state with the latest block number
            if index == logs.len() - 1 {
                STATE.with(|state| {
                    let mut s = state.borrow_mut().get().clone();
                    s.evm_block_height = event.blockNumber.unwrap();
                    state.borrow_mut().set(s).expect("Failed to set state");
                });
            }
        }
    }
}

async fn process_job(job_id: u64) {
    ic_cdk::print(format!("Processing job {}", job_id));
    let result = "42";
    submit_result(job_id, result).await;
}

async fn submit_result(job_id: u64, result: &str) {
    let config = CONFIG.with(|config| config.borrow().get().clone());
    let state = STATE.with(|state| state.borrow().get().clone());

    let function_signature = "callback(string)";
    let argument = result.to_string();

    // Encode the function call
    let mut data = keccak256(function_signature).as_ref()[0..4].to_vec();
    data.extend(ethers_core::abi::AbiEncode::encode(argument));

    //TODO: Proper fee estimation
    let fee_history = evm_rpc::fee_history(config.evm_network.clone(), 10, BlockTag::Latest, None).await;
    let base_fee = fee_history.baseFeePerGas.last().unwrap();
    // let priority_fees: Vec<_> = fee_history.reward.iter().flatten().collect();
    // let median_priority_fee = priority_fees[priority_fees.len() / 2];

    let max_priority_fee_per_gas = 100;
    let max_fee_per_gas = base_fee.saturating_add(max_priority_fee_per_gas);

    //TODO: Set chain_id based on evm_network
    let req = evm_signer::SignRequest {
        chain_id: 11155111,
        to: config.evm_contract.clone().unwrap(),
        gas: U256::from(50000),
        max_fee_per_gas: U256::from(max_fee_per_gas),
        max_priority_fee_per_gas: U256::from(max_priority_fee_per_gas),
        data: Some(data),
        value: U256::from(0),
        nonce: U256::from(state.nonce),
    };

    let tx = evm_signer::sign_transaction(req).await;

    let status = evm_rpc::send_raw_transaction(config.evm_network.clone(), tx.clone()).await;

    ic_cdk::print(format!("Transaction sent: {:?}", tx));

    match status {
        SendRawTransactionStatus::Ok => {
            ic_cdk::print("Transaction sent");
            STATE.with(|state| {
                let mut s = state.borrow_mut().get().clone();
                s.nonce += 1;
                state.borrow_mut().set(s).expect("Failed to set state");
            });
        }
        SendRawTransactionStatus::NonceTooLow => {
            ic_cdk::print("Nonce too low");
        }
        SendRawTransactionStatus::NonceTooHigh => {
            ic_cdk::print("Nonce too high");
        }
        SendRawTransactionStatus::InsufficientFunds => {
            ic_cdk::print("Insufficient funds");
        }
    }
}

fn hex_to_u64(hex_str: &str) -> Result<u64, hex::FromHexError> {
    // Remove the 0x prefix if it exists
    let clean_hex_str = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };

    let bytes = hex::decode(clean_hex_str)?;
    let mut result: u64 = 0;

    // Iterate over the bytes and construct the number
    for byte in bytes {
        result = (result << 8) | (byte as u64);
    }

    Ok(result)
}
