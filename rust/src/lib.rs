use std::{ffi::{CString, NulError}, ptr::NonNull, sync::Arc};

use bitcoinkernel_sys::*;

/// Struct for transaction operations
pub struct Transaction {
    inner: NonNull<kernel_Transaction>,
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe { kernel_transaction_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for scriptpubkey operations
pub struct ScriptPubkey {
    inner: NonNull<kernel_ScriptPubkey>,
}

impl Drop for ScriptPubkey {
    fn drop(&mut self) {
        unsafe { kernel_script_pubkey_destroy(self.inner.as_ptr()); }
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
    inner: NonNull<kernel_TransactionOutput>,
}

impl Drop for TransactionOutput {
    fn drop(&mut self) {
        unsafe { kernel_transaction_output_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for logger operations
pub struct Logger {
    
}

/// Struct for kernel notifications
pub struct KernelNotifications {
    
}

/// Struct for unowned block
pub struct UnownedBlock {
    inner: NonNull<kernel_BlockPointer>,
}

/// Struct for Block validation state
pub struct BlockValidationState {
    
}

/// Struct for validation interface
pub struct ValidationInterface {
    
}


/// Struct for ChainParams
pub struct ChainParams {
    inner: NonNull<kernel_ChainParameters>,
}

impl Drop for ChainParams {
    fn drop(&mut self) {
        unsafe { kernel_chain_parameters_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for ContextOptions
pub struct ContextOptions {
    inner: NonNull<kernel_ContextOptions>,
}

impl Drop for ContextOptions {
    fn drop(&mut self) {
        unsafe { kernel_context_options_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Context
pub struct Context {
    inner: NonNull<kernel_Context>,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { kernel_context_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Chainstate Manager Options
pub struct ChainstateManagerOptions {
    inner: NonNull<kernel_ChainstateManagerOptions>,
}

impl Drop for ChainstateManagerOptions {
    fn drop(&mut self) {
        unsafe { kernel_chainstate_manager_options_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Block Operations
pub struct Block {
    inner: NonNull<kernel_Block>,
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe { kernel_block_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Block Undo
pub struct BlockUndo {
    inner: NonNull<kernel_BlockUndo>,
}

impl Drop for BlockUndo {
    fn drop(&mut self) {
        unsafe { kernel_block_undo_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for BlockIndex
pub struct BlockIndex {
    inner: NonNull<kernel_BlockIndex>,
}

impl Drop for BlockIndex {
    fn drop(&mut self) {
        unsafe { kernel_block_index_destroy(self.inner.as_ptr()); }
    }
}

/// Struct for Chain Man
pub struct ChainMan {
    inner: NonNull<kernel_ChainstateManager>,
    context: Arc<Context>,
}

impl Drop for ChainMan {
    fn drop(&mut self) {
        unsafe { kernel_chainstate_manager_destroy(self.inner.as_ptr(), self.context.inner.as_ptr()); }
    }
}

/// A collection of errors emitted by this library
#[derive(Debug)]
pub enum KernelError {
    Internal(String),
    CStringCreationFailed(String),
    InvalidOptions(String),
    OutOfBounds,
    Scriptverify(ScriptVerifyError),
}

/// A collection of errors that may occur during scriptverification
#[derive(Debug)]
pub enum ScriptVerifyError {
    TxInputIndex,
    TxSizeMismatch,
    TxDeserialize,
    InvalidFlags,
    InvalidFlagsCombination,
    SpentOutputsMismatch,
    SpendOutputsRequired,
    Invalid,
}














