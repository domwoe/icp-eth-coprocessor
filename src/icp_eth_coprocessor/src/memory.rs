use candid::{CandidType, Decode, Deserialize, Encode};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, Storable};

use std::{borrow::Cow, cell::RefCell};

type VMem = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Deserialize, Clone)]

pub struct Config {
    pub evm_contract: Option<String>,
    pub ecdsa_key_name: String,
    pub evm_network: String,
}

impl Storable for Config {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    pub ecdsa_pub_key: Option<Vec<u8>>,
    pub evm_address: Option<String>,
    pub evm_block_height: u128,
    pub nonce: u128,
}

impl Storable for State {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

thread_local! {

    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableCell` with `MemoryId(0)`.
    pub static CONFIG: RefCell<StableCell<Config, VMem>> = RefCell::new(
            StableCell::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
                Config {
                    evm_contract: None,
                    ecdsa_key_name: "dfx_test_key_1".to_string(),
                    evm_network: "EthSepolia".to_string(),
                }
        ).unwrap()
    );

    pub static STATE: RefCell<StableCell<State, VMem>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            State {
                ecdsa_pub_key: None,
                evm_address: None,
                evm_block_height: 5552046,
                nonce: 0,
            }
    ).unwrap()
);

}
