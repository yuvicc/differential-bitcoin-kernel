use std::{ffi::{CString, NulError}, ptr::NonNull, sync::Arc};

use bitcoinkernel_sys::*;

pub mod enums;
pub mod error;
pub mod block;
pub mod transaction;
pub mod chain;
pub mod constants;

/// Struct for transaction operations
pub struct Transaction {
    inner: NonNull<btck_Transaction>,
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe { btck_transaction_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for scriptpubkey operations
pub struct ScriptPubkey {
    inner: NonNull<btck_ScriptPubkey>,
}

impl Drop for ScriptPubkey {
    fn drop(&mut self) {
        unsafe { btck_script_pubkey_destroy(self.inner.as_ptr()); }
    }
}

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

/// Struct for Transaction Output operations
pub struct TransactionOutput {
    inner: NonNull<btck_TransactionOutput>,
}

impl Drop for TransactionOutput {
    fn drop(&mut self) {
        unsafe { btck_transaction_output_destroy(self.inner.as_ptr()); }
    }
}

pub struct TransactionSpentOutputs {
    inner: NonNull<btck_BlockSpentOutputs>,
}

impl Drop for TransactionSpentOutputs {
    fn drop(&mut self) {
        unsafe { btck_block_spent_outputs_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for logger operations
pub struct Logger<T> {
    log: T,
    inner: NonNull<btck_LoggingConnection>
}

impl<T> Drop for Logger<T> {
    fn drop(&mut self) {
      unsafe { btck_logging_connection_destroy(self.inner.as_ptr()); }  
    }
}

pub fn disable_logging() {
    unsafe {
        btck_logging_disable();
    }
}

/// Struct for kernel notifications
pub struct KernelNotifications {
    
}

/// Struct for ChainParams
pub struct ChainParams {
    inner: NonNull<btck_ChainParameters>,
}

impl Drop for ChainParams {
    fn drop(&mut self) {
        unsafe { btck_chain_parameters_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for ContextOptions
pub struct ContextOptions {
    inner: NonNull<btck_ContextOptions>,
}

impl Drop for ContextOptions {
    fn drop(&mut self) {
        unsafe { btck_context_options_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Context
pub struct Context {
    inner: NonNull<btck_Context>,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { btck_context_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Chainstate Manager Options
pub struct ChainstateManagerOptions {
    inner: NonNull<btck_ChainstateManagerOptions>,
}

impl Drop for ChainstateManagerOptions {
    fn drop(&mut self) {
        unsafe { btck_chainstate_manager_options_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Block Operations
pub struct Block {
    inner: NonNull<btck_Block>,
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe { btck_block_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Chain Man
pub struct ChainstateManager {
    inner: NonNull<btck_ChainstateManager>,
    context: Arc<Context>,
}

impl Drop for ChainstateManager {
    fn drop(&mut self) {
        unsafe { btck_chainstate_manager_destroy(self.inner.as_ptr()); }
    }
}

pub struct Coin {
    inner: NonNull<btck_Coin>,
}

impl Drop for Coin {
    fn drop(&mut self) {
        unsafe { btck_coin_destroy(self.inner.as_ptr()); }
    }
}










