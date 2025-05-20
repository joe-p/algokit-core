#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

// For wasm targets, use dlmalloc
#[cfg(target_arch = "wasm32")]
mod allocator {
    use dlmalloc::GlobalDlmalloc;

    #[global_allocator]
    static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;
}

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
