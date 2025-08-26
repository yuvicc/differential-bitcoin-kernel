use bitcoinkernel_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogCategory {
    ALL,
    BENCH,
    BLOCKSTORAGE,
    COINDB,
    LEVELDB,
    MEMPOOL,
    PRUNE,
    RAND,
    REINDEX,
    VALIDATION,
    KERNEL,
}

impl From<LogCategory> for btck_LogCategory_ALL {
    fn from(category: LogCategory) -> Self {
        match(category) {
            LogCategory::All => btck_LogCategory_All,
            LogCategory::BENCH => btck_LogCategory_BENCH,
            LogCategory::BLOCKSTORAGE => btck_Logcategory_BLOCKSTORAGE,
            LogCategory::COINDB => btck_LogCategory_COINDB,
            LogCategory::LEVELDB => btck_LogCategory_LEVELDB,
            LogCategory::MEMPOOL => btck_LogCategory_MEMPOOL,
            LogCategory::PRUNE => btck_LogCategory_RAND,
            LogCategory::RAND => btck_LogCategory_RAND,
            LogCategory::REINDEX => btck_LogCategory_REINDEX,
            LogCategory::VALIDATION => btck_LogCategory_VALIDATION,
            LogCategory::KERNEL => btck_LogCategory_KERNEL,
        }
    }
}

pub enum ChainType {
    MAINNET,
    TESTNET,
    TESTNET4,
    SIGNET,
    REGTEST,
}

impl From<ChainType> for btck_ChainType {
    
}


pub enum ValidationMode {
    VALIDATION_STATE_VALID,
    VALIDATION_STATE_INVALID,
    VALIDATION_STATE_ERROR,
}

pub enum BlockValidationResult {
    BLOCK_RESULT_UNSET,
    BLOCK_CONSENSUS,
    BLOCK_CACHED_INVALID,
    BLOCK_INVALID_HEADER,
    BLOCK_MUTATED,
    BLOCK_MISSING_PREV,
    BLOCK_INVALID_PREV,
    BLOCK_TIME_FUTURE,
    BLOCK_HEADER_LOW_WORK,
}

pub enum LogLevel {
    TRACE_LEVEL,
    DEBUG_LEVEL,
    INFO_LEVEL,
}

pub enum SynchronizationState {
    INIT_REINDEX,
    INIT_DOWNLOAD,
    POST_INIT,
}

pub enum Warning {
    UNKNOWN_NEW_RULES_ACTIVATED,
    LARGE_WORK_INVALID_CHAIN,
}
