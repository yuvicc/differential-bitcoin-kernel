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