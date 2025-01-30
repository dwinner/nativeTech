cmake_minimum_required(VERSION 3.28)
project("auto_declarations")
set(CMAKE_CXX_STANDARD 23)

set_source_files_properties(simple_joys.cpp PROPERTIES COMPILE_FLAGS -std=c++11)
set_source_files_properties(auto.cpp PROPERTIES COMPILE_FLAGS -std=c++14)

add_executable(auto auto.cpp)

add_library(simple_joys simple_joys.cpp)
