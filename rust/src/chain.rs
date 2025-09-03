use bitcoinkernel_sys::*;


/// Struct for Chainstate Manager
pub struct ChainstateManager {
    inner: NonNull<btck_ChainstateManager>,
    context: Arc<Context>,
}

impl Drop for ChainstateManager {
    fn drop(&mut self) {
        unsafe { btck_chainstate_manager_destroy(self.inner.as_ptr()); }
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

/// Struct for ContextOptions
pub struct ContextOptions {
    inner: NonNull<btck_ContextOptions>,
}

impl Drop for ContextOptions {
    fn drop(&mut self) {
        unsafe { btck_context_options_destroy(self.inner.as_ptr()); }
    }
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

/// Struct for Context
pub struct Context {
    inner: NonNull<btck_Context>,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { btck_context_destroy(self.inner.as_ptr()); }
    }
}