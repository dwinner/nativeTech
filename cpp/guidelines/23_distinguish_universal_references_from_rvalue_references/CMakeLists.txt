cmake_minimum_required(VERSION 3.28)
project("universal_and_rvalue")
set(CMAKE_CXX_STANDARD 23)


# TODO: use Boost TypeIndex instead.
include_directories(../../3rd_party)

set_source_files_properties(record_time.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")

add_executable(urefs_initialization urefs_initialization.cpp)
add_executable(urefs_form urefs_form.cpp)

add_library(item24 rrefs_examples.cpp
                   urefs_contexts.cpp
                   no_type_deduction.cpp
                   record_time.cpp)
