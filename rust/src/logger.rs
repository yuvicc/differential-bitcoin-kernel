use bitcoinkernel_sys::{
    btck_
};

use crate::{enums::{LogCategory, LogLevel}, error::KernelError};





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

impl Logger {

    pub fn new() -> Result<Logger, KernelError> {
        Ok(())
    }

    pub fn set_level_category(&self, category: LogCategory, level: LogLevel) {
        unsafe {
            btck_logging_enable
        }
    }

    pub fn enable_category() {

    }

    pub fn disable_category() {

    }


}





/// Struct for kernel notifications
pub struct KernelNotifications {
    
}