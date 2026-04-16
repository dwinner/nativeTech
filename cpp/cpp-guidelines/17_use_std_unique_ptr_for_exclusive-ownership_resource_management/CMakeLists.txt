cmake_minimum_required(VERSION 3.28)
project("exclusive_ownership")
set(CMAKE_CXX_STANDARD 23)

set_source_files_properties(
    investments_cpp11.cpp
    PROPERTIES COMPILE_FLAGS -std=c++11
)
set_source_files_properties(
    investments_cpp14.cpp
    PROPERTIES COMPILE_FLAGS -std=c++14
)

add_executable(investments_cpp11 investments_cpp11.cpp)
add_executable(investments_cpp14 investments_cpp14.cpp)
