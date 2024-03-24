use candid::{CandidType, Deserialize, Nat};

use ethers_core::abi::ethereum_types::{Address, U256, U64};
use ethers_core::types::transaction::eip1559::Eip1559TransactionRequest;
use ethers_core::types::{Bytes, Signature};
use ethers_core::utils::keccak256;

use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
    SignWithEcdsaArgument,
};
use serde::Serialize;
use std::str::FromStr;

use crate::memory::STATE;

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignatureReply {
    pub signature: Vec<u8>,
}

#[derive(Deserialize)]
pub struct SignRequest {
    pub chain_id: u64,
    pub to: String,
    pub gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub value: U256,
    pub nonce: U256,
    pub data: Option<Vec<u8>>,
}

pub async fn get_public_key() -> Vec<u8> {
    let (key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: [].to_vec(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: "dfx_test_key".to_string(),
        },
    })
    .await
    .expect("failed to get public key");
    key.public_key
}

pub async fn sign_transaction(req: SignRequest) -> String {
    const EIP1559_TX_ID: u8 = 2;

    let data = req.data.as_ref().map(|d| Bytes::from(d.clone()));

    let tx = Eip1559TransactionRequest {
        chain_id: Some(U64::from(req.chain_id)),
        from: None,
        to: Some(
            Address::from_str(&req.to)
                .expect("failed to parse the destination address")
                .into(),
        ),
        gas: Some(req.gas),
        value: Some(req.value),
        nonce: Some(req.nonce),
        data,
        access_list: Default::default(),
        max_priority_fee_per_gas: Some(req.max_priority_fee_per_gas),
        max_fee_per_gas: Some(req.max_fee_per_gas),
    };

    let mut unsigned_tx_bytes = tx.rlp().to_vec();
    unsigned_tx_bytes.insert(0, EIP1559_TX_ID);

    let txhash = keccak256(&unsigned_tx_bytes);

    let signature = sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash: txhash.to_vec(),
        derivation_path: [].to_vec(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            // TODO: use the correct key id
            name: "dfx_test_key".to_string(),
        },
    })
    .await
    .expect("failed to sign the transaction")
    .0
    .signature;

    let pubkey = STATE
        .with(|state| {
            let s = state.borrow();
            let s = s.get();
            s.ecdsa_pub_key.clone()
        })
        .expect("failed to get the public key");

    let signature = Signature {
        v: y_parity(&txhash, &signature, &pubkey),
        r: U256::from_big_endian(&signature[0..32]),
        s: U256::from_big_endian(&signature[32..64]),
    };

    let mut signed_tx_bytes = tx.rlp_signed(&signature).to_vec();
    signed_tx_bytes.insert(0, EIP1559_TX_ID);

    format!("0x{}", hex::encode(&signed_tx_bytes))
}

/// Converts the public key bytes to an Ethereum address with a checksum.
pub fn pubkey_bytes_to_address(pubkey_bytes: &[u8]) -> String {
    use ethers_core::k256::elliptic_curve::sec1::ToEncodedPoint;
    use ethers_core::k256::PublicKey;

    let key =
        PublicKey::from_sec1_bytes(pubkey_bytes).expect("failed to parse the public key as SEC1");
    let point = key.to_encoded_point(false);
    // we re-encode the key to the decompressed representation.
    let point_bytes = point.as_bytes();
    assert_eq!(point_bytes[0], 0x04);

    let hash = keccak256(&point_bytes[1..]);

    ethers_core::utils::to_checksum(&Address::from_slice(&hash[12..32]), None)
}

/// Computes the parity bit allowing to recover the public key from the signature.
fn y_parity(prehash: &[u8], sig: &[u8], pubkey: &[u8]) -> u64 {
    use ethers_core::k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    let orig_key = VerifyingKey::from_sec1_bytes(pubkey).expect("failed to parse the pubkey");
    let signature = Signature::try_from(sig).unwrap();
    for parity in [0u8, 1] {
        let recid = RecoveryId::try_from(parity).unwrap();
        let recovered_key = VerifyingKey::recover_from_prehash(prehash, &signature, recid)
            .expect("failed to recover key");
        if recovered_key == orig_key {
            return parity as u64;
        }
    }

    panic!(
        "failed to recover the parity bit from a signature; sig: {}, pubkey: {}",
        hex::encode(sig),
        hex::encode(pubkey)
    )
}

fn decode_hex(hex: &str) -> Bytes {
    Bytes::from(hex::decode(hex.trim_start_matches("0x")).expect("failed to decode hex"))
}
