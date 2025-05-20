extern crate alloc;

use crate::address::Address;
use crate::constants::{Byte32, ALGORAND_CHECKSUM_BYTE_LENGTH, HASH_BYTES_LENGTH};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use sha2::{Digest, Sha512_256};

pub fn sort_msgpack_value(value: rmpv::Value) -> rmpv::Value {
    match value {
        rmpv::Value::Map(m) => {
            let mut sorted_map: BTreeMap<String, rmpv::Value> = BTreeMap::new();

            // Convert and sort all key-value pairs
            for (k, v) in m {
                if let rmpv::Value::String(key) = k {
                    let key_str = key.into_str().unwrap_or_default();
                    sorted_map.insert(key_str, sort_msgpack_value(v));
                }
            }

            // Convert back to rmpv::Value::Map
            rmpv::Value::Map(
                sorted_map
                    .into_iter()
                    .map(|(k, v)| (rmpv::Value::String(k.into()), v))
                    .collect(),
            )
        }
        rmpv::Value::Array(arr) => {
            rmpv::Value::Array(arr.into_iter().map(sort_msgpack_value).collect())
        }
        // For all other types, return as-is
        v => v,
    }
}

pub fn is_zero(n: &u64) -> bool {
    *n == 0u64
}

pub fn is_zero_addr(addr: &Address) -> bool {
    addr.pub_key == [0u8; 32]
}

pub fn is_zero_addr_opt(addr: &Option<Address>) -> bool {
    addr.as_ref().map_or(true, is_zero_addr)
}

pub fn is_empty_bytes32(bytes: &Byte32) -> bool {
    bytes == &[0u8; 32]
}

pub fn is_empty_bytes32_opt(bytes: &Option<Byte32>) -> bool {
    bytes.as_ref().map_or(true, is_empty_bytes32)
}

pub fn is_empty_string_opt(string: &Option<String>) -> bool {
    string.as_ref().map_or(true, String::is_empty)
}

pub fn is_empty_vec_opt<T>(vec: &Option<Vec<T>>) -> bool {
    vec.as_ref().map_or(true, Vec::is_empty)
}

pub fn pub_key_to_checksum(pub_key: &Byte32) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
    let mut hasher = Sha512_256::new();
    hasher.update(&pub_key);

    let mut checksum = [0u8; ALGORAND_CHECKSUM_BYTE_LENGTH];
    checksum
        .copy_from_slice(&hasher.finalize()[(HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH)..]);
    checksum
}
