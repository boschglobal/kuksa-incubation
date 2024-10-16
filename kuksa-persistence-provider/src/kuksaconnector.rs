use crate::storage;

pub fn get_from_storage_and_set(storage: &impl storage::Storage, vsspath: &str) {
    let value = storage.get(vsspath);
    println!("Got from storage: {}: {}", value,value);
}

