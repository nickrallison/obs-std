---
aliases: 
tags:
  - coding
  - rust
  - cmake
bad_links:
---
# Building Rust with CMake

The Top level CMake file should look like this:
```cmake
cmake_minimum_required(VERSION 3.12)
project(Cpp_Rust)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_BUILD_WITH_INSTALL_RPATH True)

# Add your subdirectories here
add_subdirectory(cpp_exe)
add_subdirectory(rust_lib)
```

And add this line to your CMakePresets file: environment vars
`"CARGO_TARGET_DIR": "${sourceDir}/build/default/cargo"`

Rust Lib CMake File:
```cmake
cmake_minimum_required(VERSION 3.12)
project(rust_lib)

add_custom_target(${PROJECT_NAME}
    DEPENDS "${CMAKE_BINARY_DIR}/cargo/release/librust_lib.so"
)

# Add rust_example as a CMake target
add_custom_command(
  OUTPUT "${CMAKE_BINARY_DIR}/cargo/release/librust_lib.so"
  COMMAND cargo build --release
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)
```

CPP Exe CMake File:
```cmake
cmake_minimum_required(VERSION 3.12)
project(cpp_exe)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add your source files here
set(SOURCES
    src/main.cpp
)

# Add any additional include directories here
include_directories(include)

# Add any additional library directories here
# link_directories(lib)

# Add any additional libraries here
set(LIBRARIES
    # mylib
)

# Create the executable
add_executable(${PROJECT_NAME} ${SOURCES})

add_dependencies(${PROJECT_NAME}
    rust_lib
)

# Specify link libraries
target_link_libraries(${PROJECT_NAME}
    # debug "${CMAKE_SOURCE_DIR}/rust_lib/target/debug/librust_lib.so"
    optimized "${CMAKE_BINARY_DIR}/cargo/release/librust_lib.so"
)
```