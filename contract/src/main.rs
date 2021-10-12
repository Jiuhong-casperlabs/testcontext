#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, RuntimeArgs,
};

#[no_mangle]
pub extern "C" fn contractcontext() {
    runtime::put_key("key1", storage::new_uref("value1").into());
}

#[no_mangle]
pub extern "C" fn sessioncontext() {
    runtime::put_key("key2", storage::new_uref("value2").into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "contractcontext",
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "sessioncontext",
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("contexttestcontract", contract_hash.into());
}
