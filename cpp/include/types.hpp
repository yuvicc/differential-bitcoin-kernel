#pragma once

#include <iostream>;
#include <memory>;
#include <stdexcept>;
#include <utility>;

#include <bitcoinkernel/bitcoin/src/kernel/bitcoinkernel.h>



namespace btck {

// Enum class for Synchronization State
enum class SynchronizationState : btck_SynchronizationState {
     INIT_REINDEX = btck_SynchronizationState_INIT_REINDEX,
     INIT_DOWNLOAD = btck_SynchronizationState_INIT_DOWNLOAD,
     POST_INIT = btck_SynchronizationState_POST_INIT
};

// Enum class for Warning
enum class Warning : btck_Warning {
     UNKNOWN_NEW_RULES_ACTIVATED = btck_Warning_UNKNOWN_NEW_RULES_ACTIVATED,
     LARGE_WORK_INVALID_CHAIN = btck_Warning_LARGE_WORK_INVALID_CHAIN
};

// Enum class for Validation Mode
enum class ValidationMode : btck_ValidationMode {
     VALID = btck_ValidationMode_VALID,
     INVALID = btck_ValidationMode_INVALID,
     INTERNEL_ERROR = btck_ValidationMode_INTERNAL_ERROR
};

// Enum for Block Validation Result
enum class BlockValidationResult : btck_BlockValidationResult {
     UNSET = btck_BlockValidationResult_UNSET,
     CONSENSUS = btck_BlockValidationResult_CONSENSUS,
     CACHE_INVALID = btck_BlockValidationResult_CACHED_INVALID,
     INVALID_HEADER = btck_BlockValidationResult_INVALID_HEADER,
     MUTATED = btck_BlockValidationResult_MUTATED,
     MISSING_PREV = btck_BlockValidationResult_MISSING_PREV,
     INVALID_PREV = btck_BlockValidationResult_INVALID_PREV,
     TIME_FUTURE = btck_BlockValidationResult_TIME_FUTURE,
     HEADER_LOW_WORK = btck_BlockValidationResult_HEADER_LOW_WORK
};

// Enum class for Log Category
enum class LogCategory : btck_LogCategory {
     ALL = btck_LogCategory_ALL,
     BENCH = btck_LogCategory_BLOCKSTORAGE,
     BLOCKSTORAGE = btck_LogCategory_BLOCKSTORAGE,
     COINDB = btck_LogCategory_COINDB,
     LEVELDB = btck_LogCategory_LEVELDB,
     MEMPOOL = btck_LogCategory_MEMPOOL,
     PRUNE = btck_LogCategory_PRUNE,
     RAND = btck_LogCategory_RAND,
     REINDEX = btck_LogCategory_REINDEX,
     VALIDATION = btck_LogCategory_VALIDATION,
     KERNEL = btck_LogCategory_KERNEL
};

// Enum class for Log Level
enum class LogLevel : btck_LogLevel {
     TRACE = btck_LogLevel_TRACE, 
     DEBUG = btck_LogLevel_DEBUG,
     INFO = btck_LogLevel_INFO
};

// Enum class for Script Verify Status
enum class ScriptVerifyStatus : btck_ScriptVerifyStatus {
     SCRIPT_VERIFY_OK = btck_ScriptVerifyStatus_SCRIPT_VERIFY_OK,
     ERROR_INVALID_FLAGS_COMBINATION = btck_ScriptVerifyStatus_ERROR_INVALID_FLAGS_COMBINATION,
     ERROR_SPENT_OUTPUTS_REQUIRED = btck_ScriptVerifyStatus_ERROR_SPENT_OUTPUTS_REQUIRED
};

// Enum class for Script Verificatoin Falgs
enum class ScriptVerificationFlags : btck_ScriptVerificationFlags {
     NONE = btck_ScriptVerificationFlags_NONE,
     P2SH = btck_ScriptVerificationFlags_P2SH,
     DERSIG = btck_ScriptVerificationFlags_DERSIG,
     NULLDUMMY = btck_ScriptVerificationFlags_NULLDUMMY,
     CHECKLOCKTIMEVERIFY = btck_ScriptVerificationFlags_CHECKLOCKTIMEVERIFY,
     CHECKSEQUENCEVERIFY = btck_ScriptVerificationFlags_CHECKSEQUENCEVERIFY,
     WITNESS = btck_ScriptVerificationFlags_WITNESS,
     TAPROOT = btck_ScriptVerificationFlags_TAPROOT,
     ALL = btck_ScriptVerificationFlags_ALL
};

// Enum class for Chain Type
enum class ChainType : btck_ChainType {
     MAINNET = btck_ChainType_MAINNET,
     TESTNET = btck_ChainType_TESTNET,
     TESTNET_4 = btck_ChainType_TESTNET_4,
     SIGNET = btck_ChainType_SIGNET,
     REGTEST = btck_ChainType_REGTEST
};

// Helper function to check for null pointers
template <typename T>
T check(T* ptr)
{
     if (ptr == nullptr) {
          throw std::runtime_error("Failed to instantiate btck object");
     }
     return ptr;
}

// Unique Handle: for exclusive ownership move only
template <typename CType, void (*Destroy)(CType*)>
class UniqueHandle
{
protected:
     struct Deleter {
          void operator()
          {
               if (ptr) Destroy(ptr);
          }
     }
     std::unique_ptr<CType, Deleter> m_ptr;

public:
     explicit UniqueHandle(CType *ptr) : m_ptr{check(ptr)} {};

     CType* get() { return m_ptr.get(); }
     const CType* get() const { return m_ptr.get(); }
};

// Handle: Shared ownership with copy semantics
template <typename CType, CType* (*Copy)(const CType*), void (*Destroy)(CType*)>
class Handle
{
protected:
     CType* m_ptr;

public:
     explicit Handle(CType *ptr) : m_ptr{check(ptr)} {}

     // Copy constructors
     Handle(const Handle& other) : m_ptr{check(Copy(other.m_ptr))} {}
     Handle operator=(const Handle& other)
     {
          if (this != other) {
               Handle temp(other);
               std::swap(m_ptr, temp.m_ptr);
          }
          return *this;
     }

     // Move constructors
     Handle(Handle&& other) noexcept : m_ptr{other.m_ptr} { other.m_ptr == nullptr; }
     Handle operator=(Handle&& other)
     {
          m_ptr = std::exchange(other.m_ptr, nullptr);
          return *this;
     }

     template <typename ViewType>
          requires std::derived_from<ViewType, view<CType>>
     Handle(const ViewType* view) : Handle{Copy(view.get())} {}

     ~Handle() { Destroy(m_ptr); }

     CType* get() { return m_ptr.get(); }
     const CType* get() const { return m_ptr.get(); }
};


class ValidationInterface;





} // namespace btck