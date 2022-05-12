use alloc::string::String;
use alloc::vec;
use casper_types::CLType;
use casper_types::account::AccountHash;
use alloc::boxed::Box;

use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLTyped, Parameter
};

pub fn store_string_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("store_string"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn store_account_hash_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("store_account_hash"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn store_bytes_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("store_bytes"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn remove_key_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("remove_key"),
        vec![
            Parameter::new("name", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn create_dictionary_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("create_dictionary"),
        vec![
            Parameter::new("name", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn add_string_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("add_string"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn add_item_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("add_item"),
        vec![
            Parameter::new("value", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn get_string_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("get_string"),
        vec![
            Parameter::new("name", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn create_dictionary_array_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("create_dictionary_array"),
        vec![
            Parameter::new("name", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn push_item_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("push_item"),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn get_item_entry_point() -> EntryPoint {
    EntryPoint::new(
        String::from("get_item"),
        vec![
            Parameter::new("name", CLType::String),
        ],
        CLType::List(Box::new(CLType::String)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn default() -> EntryPoints {

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(store_string_entry_point());
    entry_points.add_entry_point(store_account_hash_entry_point());
    entry_points.add_entry_point(store_bytes_entry_point());
    entry_points.add_entry_point(remove_key_entry_point());
    entry_points.add_entry_point(create_dictionary_entry_point());
    entry_points.add_entry_point(add_string_entry_point());
    entry_points.add_entry_point(add_item_entry_point());
    entry_points.add_entry_point(get_string_entry_point());
    entry_points.add_entry_point(create_dictionary_array_entry_point());

    entry_points.add_entry_point(push_item_entry_point());

    entry_points.add_entry_point(get_item_entry_point());
    entry_points
}