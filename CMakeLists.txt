cmake_minimum_required(VERSION 3.16)
project(BitcoinKernelTest)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(BITCOIN_BUILD_DIR "${CMAKE_CURRENT_SOURCE_DIR}/bitcoinkernel/build")
set(BITCOIN_LIB_DIR "${BITCOIN_BUILD_DIR}/lib")
set(BITCOIN_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/bitcoinkernel/bitcoin/src")

# Check if the library exists
if(NOT EXISTS "${BITCOIN_LIB_DIR}")
    message(FATAL_ERROR "Bitcoin library directory not found at: ${BITCOIN_LIB_DIR}")
endif()

find_library(BITCOIN_KERNEL_LIB
    NAMES bitcoinkernel libbitcoinkernel
    PATHS "${BITCOIN_LIB_DIR}"
    NO_DEFAULT_PATH
)

if(NOT BITCOIN_KERNEL_LIB)
    message(FATAL_ERROR "Bitcoin kernel library not found in: ${BITCOIN_LIB_DIR}")
endif()

message(STATUS "Found Bitcoin kernel library: ${BITCOIN_KERNEL_LIB}")

add_subdirectory(test)