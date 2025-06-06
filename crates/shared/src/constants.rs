use super::types::EncodedVerifyingKey;
use hex_literal::hex;
use lazy_static::lazy_static;
use sp_core::H256;
use sp_std::vec;
use sp_std::vec::Vec;

pub const DEFAULT_VERIFYING_KEY_NOT_REGISTERED: EncodedVerifyingKey =
    [10; VERIFICATION_KEY_LENGTH as usize];
// This key is associated with a constant key share generation from DETERMINISTIC_KEY_SHARE_DAVE
pub const DAVE_VERIFYING_KEY: EncodedVerifyingKey = [
    3, 42, 97, 187, 199, 208, 95, 166, 102, 15, 38, 146, 173, 111, 175, 123, 62, 132, 178, 237,
    150, 199, 194, 240, 153, 30, 113, 104, 57, 63, 54, 2, 65,
];
// This key is associated with a constant key share generation from DETERMINISTIC_KEY_SHARE_NETWORK
pub const PREGENERATED_NETWORK_VERIFYING_KEY: EncodedVerifyingKey = [
    2, 78, 59, 129, 175, 156, 34, 52, 202, 208, 157, 103, 156, 230, 3, 94, 209, 57, 35, 71, 206,
    100, 206, 64, 95, 93, 205, 54, 34, 138, 37, 222, 110,
];
pub const FERDIE_VERIFYING_KEY: EncodedVerifyingKey = [3; VERIFICATION_KEY_LENGTH as usize];
pub const DEFAULT_VERIFYING_KEY: EncodedVerifyingKey = [0; VERIFICATION_KEY_LENGTH as usize];

lazy_static! {
    // key used to create a deterministic key share for EVE taken from here https://docs.rs/k256/latest/k256/ecdsa/index.html
    pub static ref DETERMINISTIC_KEY_SHARE_NETWORK: [u8; 32] =  hex!("4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318");
    // key used to create a deterministic key for DAVE - this is random 32 bytes
    pub static ref DETERMINISTIC_KEY_SHARE_DAVE: [u8; 32] =  hex!("06b07fd12cdfb94fbde3ff2098e9f19bb11b00959680cfbd15c914b025f298d7");
    // hash used to find DEVICE_KEY_PROXY onchain
    pub static ref DEVICE_KEY_HASH: H256 =  H256::zero();
    // Device key config struct seralized by generate types in programs repo
    pub static ref DEVICE_KEY_CONFIG_TYPE: Vec<u8> = vec![123, 34, 36, 115, 99, 104, 101, 109, 97, 34, 58, 34, 104, 116, 116, 112, 58, 47, 47, 106, 115, 111, 110, 45, 115, 99, 104, 101, 109, 97, 46, 111, 114, 103, 47, 100, 114, 97, 102, 116, 45, 48, 55, 47, 115, 99, 104, 101, 109, 97, 35, 34, 44, 34, 116, 105, 116, 108, 101, 34, 58, 34, 85, 115, 101, 114, 67, 111, 110, 102, 105, 103, 34, 44, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 74, 83, 79, 78, 45, 100, 101, 115, 101, 114, 105, 97, 108, 105, 122, 97, 98, 108, 101, 32, 115, 116, 114, 117, 99, 116, 32, 116, 104, 97, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 117, 115, 101, 100, 32, 116, 111, 32, 100, 101, 114, 105, 118, 101, 32, 116, 104, 101, 32, 112, 114, 111, 103, 114, 97, 109, 45, 74, 83, 79, 78, 32, 105, 110, 116, 101, 114, 102, 97, 99, 101, 46, 32, 78, 111, 116, 101, 32, 104, 111, 119, 32, 116, 104, 105, 115, 32, 117, 115, 101, 115, 32, 74, 83, 79, 78, 45, 110, 97, 116, 105, 118, 101, 32, 116, 121, 112, 101, 115, 32, 111, 110, 108, 121, 46, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 111, 98, 106, 101, 99, 116, 34, 44, 34, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 34, 58, 123, 34, 101, 99, 100, 115, 97, 95, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 115, 34, 58, 123, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 98, 97, 115, 101, 54, 52, 45, 101, 110, 99, 111, 100, 101, 100, 32, 99, 111, 109, 112, 114, 101, 115, 115, 101, 100, 32, 112, 111, 105, 110, 116, 32, 40, 51, 51, 45, 98, 121, 116, 101, 41, 32, 69, 67, 68, 83, 65, 32, 112, 117, 98, 108, 105, 99, 32, 107, 101, 121, 115, 44, 32, 40, 101, 103, 46, 32, 92, 34, 65, 53, 55, 50, 100, 113, 111, 117, 101, 53, 79, 121, 119, 89, 47, 52, 56, 100, 116, 121, 116, 81, 105, 109, 76, 57, 87, 79, 48, 100, 112, 83, 79, 98, 97, 70, 98, 65, 120, 111, 69, 87, 87, 57, 92, 34, 41, 34, 44, 34, 116, 121, 112, 101, 34, 58, 91, 34, 97, 114, 114, 97, 121, 34, 44, 34, 110, 117, 108, 108, 34, 93, 44, 34, 105, 116, 101, 109, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 125, 44, 34, 101, 100, 50, 53, 53, 49, 57, 95, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 91, 34, 97, 114, 114, 97, 121, 34, 44, 34, 110, 117, 108, 108, 34, 93, 44, 34, 105, 116, 101, 109, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 125, 44, 34, 115, 114, 50, 53, 53, 49, 57, 95, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 91, 34, 97, 114, 114, 97, 121, 34, 44, 34, 110, 117, 108, 108, 34, 93, 44, 34, 105, 116, 101, 109, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 125, 125, 125];
    // Device key aux data struct seralized by generate types in programs repo
    pub static ref DEVICE_KEY_AUX_DATA_TYPE: Vec<u8> = vec![123, 34, 36, 115, 99, 104, 101, 109, 97, 34, 58, 34, 104, 116, 116, 112, 58, 47, 47, 106, 115, 111, 110, 45, 115, 99, 104, 101, 109, 97, 46, 111, 114, 103, 47, 100, 114, 97, 102, 116, 45, 48, 55, 47, 115, 99, 104, 101, 109, 97, 35, 34, 44, 34, 116, 105, 116, 108, 101, 34, 58, 34, 65, 117, 120, 68, 97, 116, 97, 34, 44, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 74, 83, 79, 78, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 101, 32, 97, 117, 120, 105, 108, 105, 97, 114, 121, 32, 100, 97, 116, 97, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 111, 98, 106, 101, 99, 116, 34, 44, 34, 114, 101, 113, 117, 105, 114, 101, 100, 34, 58, 91, 34, 99, 111, 110, 116, 101, 120, 116, 34, 44, 34, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 34, 44, 34, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 95, 116, 121, 112, 101, 34, 44, 34, 115, 105, 103, 110, 97, 116, 117, 114, 101, 34, 93, 44, 34, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 34, 58, 123, 34, 99, 111, 110, 116, 101, 120, 116, 34, 58, 123, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 84, 104, 101, 32, 99, 111, 110, 116, 101, 120, 116, 32, 102, 111, 114, 32, 116, 104, 101, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 111, 110, 108, 121, 32, 110, 101, 101, 100, 101, 100, 32, 105, 110, 32, 115, 114, 50, 53, 53, 49, 57, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 116, 121, 112, 101, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 44, 34, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 34, 58, 123, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 98, 97, 115, 101, 54, 52, 45, 101, 110, 99, 111, 100, 101, 100, 32, 112, 117, 98, 108, 105, 99, 32, 107, 101, 121, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 44, 34, 112, 117, 98, 108, 105, 99, 95, 107, 101, 121, 95, 116, 121, 112, 101, 34, 58, 123, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 92, 34, 101, 99, 100, 115, 97, 92, 34, 44, 32, 92, 34, 101, 100, 50, 53, 53, 49, 57, 92, 34, 44, 32, 92, 34, 115, 114, 50, 53, 53, 49, 57, 92, 34, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 44, 34, 115, 105, 103, 110, 97, 116, 117, 114, 101, 34, 58, 123, 34, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 34, 58, 34, 98, 97, 115, 101, 54, 52, 45, 101, 110, 99, 111, 100, 101, 100, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 125, 125];
}

// min balance 10 decimal chain = 1
pub const MIN_BALANCE: u128 = 10_000_000_000;

// 6 seconds a block this is one day
/// The amount of blocks before a tx request is pruned from the kvdb
pub const PRUNE_BLOCK: u32 = 14400;

#[cfg(not(test))]
/// Timeout for validators to wait for other validators to join protocol committees
pub const SETUP_TIMEOUT_SECONDS: u64 = 20;
#[cfg(test)]
/// Timeout for validators to wait for other validators to join protocol committees
/// Made longer for testing scenerio
pub const SETUP_TIMEOUT_SECONDS: u64 = 100;

/// The amount of proactive refreshes we do per session
pub const REFRESHES_PER_SESSION: u32 = 10;

/// Max instructions per wasm program
pub const INITIAL_MAX_INSTRUCTIONS_PER_PROGRAM: u64 = 100_000_000;

/// Blocks a transaction is valid for
pub const MORTALITY_BLOCKS: u64 = 32;

/// Size of the verification key
pub const VERIFICATION_KEY_LENGTH: u32 = 33;

/// `device_key_proxy.wasm` from the `programs` repo.
pub const DEVICE_KEY_PROXY: &[u8] = include_bytes!("../device_key_proxy.wasm");

/// Network parent key specific size to fit into [u8; 32] to save extra code
pub const NETWORK_PARENT_KEY: &str = "NETWORK_PARENT_KEY_FOR_ENTROPY_";

// Lookup key for the next network parent keyshare ready to be swapped in to new one
pub const NEXT_NETWORK_PARENT_KEY: &str = "NEXT_NETWORK_PARENT_KEY";

/// Total signers on the network with the parent key
pub const TOTAL_SIGNERS: u8 = 3;

/// Max signers, for bounding of total signers for benches,
/// Can be changed but requires a re-run of benches
pub const MAX_SIGNERS: u8 = 15;

/// Threshold for those signers
pub const SIGNER_THRESHOLD: u8 = 2;

/// Program version number, must be incremented if version number changes
pub const PROGRAM_VERSION_NUMBER: u8 = 0;
