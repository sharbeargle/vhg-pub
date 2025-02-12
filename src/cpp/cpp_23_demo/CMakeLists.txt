#### To build for C++23
#    1. Install clang-19 (symlink /usr/bin/clang++ -> /usr/lib/llvm-19/bin/clang++)
#    2. Install g++-14 (which will give you libstdc++-14-dev)

#### Set your C compiler to clang before running cmake . 
# export CC=/usr/bin/clang
# export CXX=/usr/bin/clang++


cmake_minimum_required(VERSION 3.10)

project(CPP_23_DEMO)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_executable(CPP_23_DEMO main.cc)
