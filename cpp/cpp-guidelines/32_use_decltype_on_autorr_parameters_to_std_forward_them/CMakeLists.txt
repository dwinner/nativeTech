cmake_minimum_required(VERSION 3.28)
project("use_decltype_on_autorr")
set(CMAKE_CXX_STANDARD 23)

set_source_files_properties(item35.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
add_executable(item35 item35.cpp)
