cmake_minimum_required(VERSION 3.28)
project("noexcept_functions")
set(CMAKE_CXX_STANDARD 23)

add_library(never_throw_exception never_throw_exception.cpp)
add_library(example_with_move example_with_move.cpp)

add_executable(inconsistencies inconsistencies.cpp)
