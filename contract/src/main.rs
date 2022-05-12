#![no_std]
#![no_main]

#[warn(unused_imports)]
#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use alloc::vec::Vec;

use alloc::{string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::URef;
use casper_types::ContractPackageHash;
use casper_types::CLType;
use casper_types::CLTyped;
use casper_types::account::AccountHash;
use casper_types::{
    bytesrepr::ToBytes
};

mod entry_points;

 #[no_mangle]
 pub extern "C" fn store_string() {
     let name: String = runtime::get_named_arg("name");
     let value: String = runtime::get_named_arg("value");
     set_key(name.as_str(), value);
 }

 #[no_mangle]
 pub extern "C" fn store_account_hash() {
     let name: String = runtime::get_named_arg("name");
     let value: AccountHash = runtime::get_named_arg("value");
     set_key(name.as_str(), value);
 }

 #[no_mangle]
 pub extern "C" fn store_bytes() {
     let name: String = runtime::get_named_arg("name");
     let value: Vec<u8> = runtime::get_named_arg("value");
     set_key(name.as_str(), value);
 }

 #[no_mangle]
 pub extern "C" fn remove_key() {
     let name: String = runtime::get_named_arg("name");
     remove(name.as_str());
 }

 #[no_mangle]
 pub extern "C" fn create_dictionary() {
    create_dictionary_private();
 }

 #[no_mangle]
 pub extern "C" fn add_string() {
    let name: String = runtime::get_named_arg("name");
    let value: String = runtime::get_named_arg("value");
    add_string_private(name.as_str(), value);
 }

 #[no_mangle]
 pub extern "C" fn add_item() {
    let value: String = runtime::get_named_arg("value");

    add_item_private(value.as_str())
 }

 #[no_mangle]
 pub extern "C" fn get_string() {
    let name: String = runtime::get_named_arg("name");

    get_string_private(name.as_str())
 }

 #[no_mangle]
 pub extern "C" fn create_dictionary_array() {
    let name: String = runtime::get_named_arg("name");

    create_dictionary_array_private(name.as_str())
 }

 #[no_mangle]
 pub extern "C" fn push_item() {
    let name: String = runtime::get_named_arg("name");
    let value: String = runtime::get_named_arg("value");

    push_item_private(name.as_str(), value);
 }

 #[no_mangle]
 pub extern "C" fn get_item() -> Vec<String> {
    let name: String = runtime::get_named_arg("name");

    get_item_private(name.as_str())
 }

 #[no_mangle]
 pub extern "C" fn call() {
    let (contract_package_hash, access_uref): (ContractPackageHash, URef) =
    storage::create_contract_package_at_hash();

     let key = runtime::get_key("ADDRESSES_DICTIONARY_ARRAY")
         .unwrap_or_revert();

    runtime::put_key("SAVING_HASH_KEY_NAME", contract_package_hash.into());
    runtime::put_key("SAVING_ACCESS_KEY_NAME", access_uref.into());

     let mut entry_points = entry_points::default();

    let (contract_hash, contract_version) = storage::add_contract_version(
        contract_package_hash, entry_points, 
        Default::default());

    let version_uref = storage::new_uref(contract_version);
    runtime::put_key("SAVING_CONTRACT_VERSION_KEY", version_uref.into());
    runtime::put_key("SAVING_ARG_CONTRACT_HASH_NAME", contract_hash.into());

 }

 fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
     match runtime::get_key(name) {
         Some(key) => {
             let key_ref = key.try_into().unwrap_or_revert();
             storage::write(key_ref, value);
         }
         None => {
             let key = storage::new_uref(value).into();
             runtime::put_key(name, key);
         }
     }
 }
 
 fn remove(name: &str) {
     runtime::remove_key(name)
 }

 fn create_dictionary_private() {
    let addresses_uref: URef = storage::new_dictionary("ADDRESSES_DICTIONARY").unwrap_or_revert();
    runtime::put_key("ADDRESSES_DICTIONARY", addresses_uref.into())
 }


 fn add_string_private(name: &str, value: String) {
    let key = runtime::get_key("ADDRESSES_DICTIONARY")
        .unwrap_or_revert();

    //let value_uref: URef = storage::new_uref(value);
    //runtime::put_key(name, value_uref.try_into());

    storage::dictionary_put(key.try_into().unwrap_or_revert(), name, value);
 }

 fn add_item_private(value: &str) {
    let key = runtime::get_key("ADDRESSES_DICTIONARY")
        .unwrap_or_revert();

    storage::write(key.try_into().unwrap_or_revert(), value);
 }

 fn get_string_private(name: &str) {
    let key = runtime::get_key("ADDRESSES_DICTIONARY")
        .unwrap_or_revert();

    let value: String = storage::dictionary_get(key.try_into().unwrap_or_revert(), name)
        .unwrap_or_revert()
        .unwrap_or_default();

    let value_uref: URef = storage::new_uref(value);
    
    runtime::put_key(name, value_uref.into());
 }

 fn create_dictionary_array_private(name: &str) {
    let key: URef = storage::new_dictionary("ADDRESSES_DICTIONARY_ARRAY").unwrap_or_revert();
    runtime::put_key("ADDRESSES_DICTIONARY_ARRAY", key.into());

    let v: Vec<String> = Vec::new();
    storage::dictionary_put(key, name, v);
 }

 fn push_item_private(name: &str, value: String) {
    let key = runtime::get_key("ADDRESSES_DICTIONARY_ARRAY")
        .unwrap_or_revert();

    let mut list: Vec<String> = storage::dictionary_get(key.try_into().unwrap_or_revert(), name)
        .unwrap_or_revert()
        .unwrap_or_default();

    //let value_uref = storage::new_uref(value).into_read();
    list.push(value);
    
    storage::dictionary_put(key.try_into().unwrap_or_revert(), name, list);
 }

 fn get_item_private(name: &str) -> Vec<String> {
    let key = runtime::get_key("ADDRESSES_DICTIONARY_ARRAY")
        .unwrap_or_revert();

    storage::dictionary_get(key.try_into().unwrap_or_revert(), name)
        .unwrap_or_revert()
        .unwrap_or_default()
 }
