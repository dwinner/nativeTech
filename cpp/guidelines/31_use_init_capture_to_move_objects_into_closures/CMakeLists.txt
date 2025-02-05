cmake_minimum_required(VERSION 3.28)
project("init_capture")
set(CMAKE_CXX_STANDARD 23)

set_source_files_properties(init_capture_cpp14.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
add_executable(init_capture_cpp14
               init_capture_cpp14.cpp)

set_source_files_properties(init_capture_cpp11.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
add_executable(init_capture_cpp11
               init_capture_cpp11.cpp
               Widget.cpp)

set_source_files_properties(move_capture1_cpp14.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
add_executable(move_capture1_cpp14
               move_capture1_cpp14.cpp)

add_executable(move_capture1_cpp11
               move_capture1_cpp11.cpp)

set_source_files_properties(move_capture2.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
add_executable(move_capture2
               move_capture2.cpp)

