use bitcoinkernel_sys::*;


/// Struct for transaction operations
pub struct Transaction {
    inner: NonNull<btck_Transaction>,
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe { btck_transaction_destroy(self.inner.as_ptr()); }
    }
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

/// Struct for scriptpubkey operations
pub struct ScriptPubkey {
    inner: NonNull<btck_ScriptPubkey>,
}

impl Drop for ScriptPubkey {
    fn drop(&mut self) {
        unsafe { btck_script_pubkey_destroy(self.inner.as_ptr()); }
    }
}