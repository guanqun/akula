use crate::{common, dbutils};
use bytes::Bytes;

pub fn index_chunk_key(key: &[u8], block_number: u64) -> Bytes {
    match key.len() {
        // hashed state, accounts
        common::HASH_LENGTH | common::ADDRESS_LENGTH => key
            .iter()
            .chain(&block_number.to_be_bytes())
            .copied()
            .collect(),
        // hashed state storage
        dbutils::COMPOSITE_STORAGE_KEY_LENGTH => key
            [..common::HASH_LENGTH + common::HASH_LENGTH + common::BLOCK_NUMBER_LENGTH]
            .iter()
            .chain(&key[common::HASH_LENGTH + common::INCARNATION_LENGTH..])
            .chain(&block_number.to_be_bytes())
            .copied()
            .collect(),
        // plain state storage
        dbutils::PLAIN_COMPOSITE_STORAGE_KEY_LENGTH => key
            [..common::ADDRESS_LENGTH + common::HASH_LENGTH + common::BLOCK_NUMBER_LENGTH]
            .iter()
            .chain(&key[common::ADDRESS_LENGTH + common::INCARNATION_LENGTH..])
            .chain(&block_number.to_be_bytes())
            .copied()
            .collect(),
        other => {
            panic!("Unexpected length: {}", other);
        }
    }
}
