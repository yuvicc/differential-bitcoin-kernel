use bitcoinkernel_sys::*;

/// Struct for Block Operations
pub struct Block {
    inner: NonNull<btck_Block>,
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe { btck_block_destroy(self.inner.as_ptr()); }
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