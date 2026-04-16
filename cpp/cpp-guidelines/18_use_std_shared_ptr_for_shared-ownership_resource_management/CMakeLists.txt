cmake_minimum_required(VERSION 3.28)
project("using_shared")
set(CMAKE_CXX_STANDARD 23)

add_executable(shared_ptr_things_to_know shared_ptr_things_to_know.cpp)
add_library(multiple_control_blocks1 multiple_control_blocks1.cpp)
add_library(multiple_control_blocks2 multiple_control_blocks2.cpp)
