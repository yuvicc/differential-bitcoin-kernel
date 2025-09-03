use std::{ffi::{CString, NulError}, ptr::NonNull, sync::Arc};

use bitcoinkernel_sys::*;

pub mod enums;
pub mod error;
pub mod block;
pub mod transaction;
pub mod chain;
pub mod constants;

/// Verify a transaction input against its output script
pub fn verify_script(
    script_pubkey: &ScriptPubkey,
    amount: Option<i64>,
    tx_to: &Transaction,
    spent_outputs: &[TransactionOutput],
    input_index: u32,
    flags: Option<u32>
) -> Result<(), KernelError> {
    
}










