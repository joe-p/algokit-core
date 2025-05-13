#![cfg_attr(not(debug_assertions), no_std)]

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod address;
mod constants;
mod error;
mod traits;
mod transactions;
mod utils;

// Re-export all the public items
pub use address::Address;
pub use constants::{
    Byte32, ALGORAND_ADDRESS_LENGTH, ALGORAND_CHECKSUM_BYTE_LENGTH,
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, HASH_BYTES_LENGTH,
};
pub use error::AlgoKitTransactError;
pub use traits::{AlgorandMsgpack, EstimateTransactionSize, TransactionId};
pub use transactions::{
    AssetTransferTransactionBuilder, AssetTransferTransactionFields, PaymentTransactionBuilder,
    PaymentTransactionFields, SignedTransaction, Transaction, TransactionHeader,
    TransactionHeaderBuilder,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
pub mod test_utils;
