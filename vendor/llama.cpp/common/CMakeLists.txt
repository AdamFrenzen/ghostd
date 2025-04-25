# common

find_package(Threads REQUIRED)

llama_add_compile_flags()

# Skip build-info generation entirely — not needed for vendored use

set(TARGET common)

add_library(${TARGET} STATIC
    build_info.cpp # hard coded vars from issue with no .git
    arg.cpp
    arg.h
    base64.hpp
    chat.cpp
    chat.h
    common.cpp
    common.h
    console.cpp
    console.h
    json-schema-to-grammar.cpp
    json.hpp
    llguidance.cpp
    log.cpp
    log.h
    minja/chat-template.hpp
    minja/minja.hpp
    ngram-cache.cpp
    ngram-cache.h
    sampling.cpp
    sampling.h
    speculative.cpp
    speculative.h
    )

if (BUILD_SHARED_LIBS)
    set_target_properties(${TARGET} PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()

set(LLAMA_COMMON_EXTRA_LIBS "")  # build_info removed

# Use curl to download model url
if (LLAMA_CURL)
    find_package(CURL)
    if (NOT CURL_FOUND)
        message(FATAL_ERROR "Could NOT find CURL. Hint: to disable this feature, set -DLLAMA_CURL=OFF")
    endif()
    target_compile_definitions(${TARGET} PUBLIC LLAMA_USE_CURL)
    include_directories(${CURL_INCLUDE_DIRS})
    find_library(CURL_LIBRARY curl REQUIRED)
    set(LLAMA_COMMON_EXTRA_LIBS ${LLAMA_COMMON_EXTRA_LIBS} ${CURL_LIBRARY})
endif ()

if (LLAMA_LLGUIDANCE)
    include(ExternalProject)
    set(LLGUIDANCE_SRC ${CMAKE_BINARY_DIR}/llguidance/source)
    set(LLGUIDANCE_PATH ${LLGUIDANCE_SRC}/target/release)

    # Set the correct library file extension based on platform
    if (WIN32)
        set(LLGUIDANCE_LIB_NAME "llguidance.lib")
        # Add Windows-specific libraries
        set(LLGUIDANCE_PLATFORM_LIBS
            ws2_32    # Windows Sockets API
            userenv   # For GetUserProfileDirectoryW
            ntdll     # For NT functions
            bcrypt    # For BCryptGenRandom
        )
    else()
        set(LLGUIDANCE_LIB_NAME "libllguidance.a")
        set(LLGUIDANCE_PLATFORM_LIBS "")
    endif()

    ExternalProject_Add(llguidance_ext
        GIT_REPOSITORY https://github.com/guidance-ai/llguidance
        # v0.7.10:
        GIT_TAG 0309d2a6bf40abda35344a362edc71e06d5009f8
        PREFIX ${CMAKE_BINARY_DIR}/llguidance
        SOURCE_DIR ${LLGUIDANCE_SRC}
        BUILD_IN_SOURCE TRUE
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build --release
        INSTALL_COMMAND ""
        BUILD_BYPRODUCTS ${LLGUIDANCE_PATH}/${LLGUIDANCE_LIB_NAME} ${LLGUIDANCE_PATH}/llguidance.h
        UPDATE_COMMAND ""
    )
    target_compile_definitions(${TARGET} PUBLIC LLAMA_USE_LLGUIDANCE)

    add_library(llguidance STATIC IMPORTED)
    set_target_properties(llguidance PROPERTIES IMPORTED_LOCATION ${LLGUIDANCE_PATH}/${LLGUIDANCE_LIB_NAME})
    add_dependencies(llguidance llguidance_ext)

    target_include_directories(${TARGET} PRIVATE ${LLGUIDANCE_PATH})
    # Add platform libraries to the main target
    set(LLAMA_COMMON_EXTRA_LIBS ${LLAMA_COMMON_EXTRA_LIBS} llguidance ${LLGUIDANCE_PLATFORM_LIBS})
endif ()

target_include_directories(${TARGET} PUBLIC .)
target_compile_features   (${TARGET} PUBLIC cxx_std_17)
target_link_libraries     (${TARGET} PRIVATE ${LLAMA_COMMON_EXTRA_LIBS} PUBLIC llama Threads::Threads)
