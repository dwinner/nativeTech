cmake_minimum_required(VERSION 3.28)
project("avoid_default_capture")
set(CMAKE_CXX_STANDARD 23)

set_source_files_properties(by_reference_capture.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")

add_executable(by_reference_capture
               Widget.cpp utils.cpp by_reference_capture.cpp)
