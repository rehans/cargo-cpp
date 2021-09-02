// Constants
pub const CMAKELISTS_TXT_NAME: &str = "CMakeLists.txt";
pub const CMAKELISTS_TXT_CONTENTS: &str = "
    cmake_minimum_required(VERSION @CMAKE_MINIMUM_VERSION@)

    project(@CMAKE_PROJECT_NAME@)

    add_library(@CMAKE_TARGET_NAME@ STATIC
        # @INCLUDE_DIR@/@INCLUDE_DOMAIN_DIR@/@CMAKE_TARGET_NAME@.h
        # @SOURCE_DIR@/@CMAKE_TARGET_NAME@.cpp
    )

    target_include_directories(@CMAKE_TARGET_NAME@
        PUBLIC
            ${CMAKE_CURRENT_LIST_DIR}/@INCLUDE_DIR@
        PRIVATE
            ${CMAKE_CURRENT_LIST_DIR}/@SOURCE_DIR@
    )
";
