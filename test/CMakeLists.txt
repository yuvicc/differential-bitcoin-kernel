add_executable(test_c
    test.c
)

target_include_directories(test_c PRIVATE
    ${BITCOIN_INCLUDE_DIR}
    ${BITCOIN_INCLUDE_DIR}/kernel
)

target_link_libraries(test_c PRIVATE
    ${BITCOIN_KERNEL_LIB}
)

set_target_properties(test_c PROPERTIES
    INSTALL_RPATH "${BITCOIN_LIB_DIR}"
    BUILD_WITH_INSTALL_RPATH TRUE
)

message(STATUS "Linking test_c against: ${BITCOIN_KERNEL_LIB}")
message(STATUS "Include directories: ${BITCOIN_INCLUDE_DIR} and ${BITCOIN_INCLUDE_DIR}/kernel")