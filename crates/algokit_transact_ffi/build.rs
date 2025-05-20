include!("src/lib.rs");

fn main() {
    #[cfg(test)]
    generate_test_data()
}

#[cfg(test)]
fn generate_test_data() {
    use algokit_transact::test_utils;
    use serde::Serialize;
    use std::path::Path;

    #[derive(Serialize)]
    struct TransactionTestData {
        signing_private_key: [u8; 32],
        transaction: Transaction,
        unsigned_bytes: Vec<u8>,
        signed_bytes: Vec<u8>,
        id: String,
        raw_id: [u8; 32],
    }

    test_utils::TestDataMother::export(
        Path::new("./test_data.json"),
        Some(|d: &test_utils::TransactionTestData| TransactionTestData {
            signing_private_key: d.signing_private_key,
            transaction: d.transaction.clone().try_into().unwrap(),
            unsigned_bytes: d.unsigned_bytes.clone(),
            signed_bytes: d.signed_bytes.clone(),
            id: d.id.clone(),
            raw_id: d.raw_id,
        }),
    );
}
