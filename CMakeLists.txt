cmake_minimum_required(VERSION 3.10)

project(neSia)

# Specify the path to LLVM's root directory
set(LLVM_DIR "/path/to/llvm/share/llvm/cmake")

find_package(LLVM REQUIRED CONFIG)

add_executable(nesia src/main.cpp)

# Link LLVM libraries
target_link_libraries(nesia PRIVATE LLVM)
