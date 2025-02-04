cmake_minimum_required(VERSION 3.28)
project("familirize")
set(CMAKE_CXX_STANDARD 23)


set_source_files_properties(constraining_templates02_cpp14.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
set_source_files_properties(constraining_templates03_cpp14.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")
set_source_files_properties(constraining_templates04_cpp14.cpp PROPERTIES COMPILE_FLAGS "-std=c++23")

add_library(alternatives_to_uref_overloading alternative_pass_by_value.cpp
                                             tag_dispatch01.cpp
                                             tag_dispatch02.cpp
                                             constraining_templates01.cpp
                                             constraining_templates02_cpp11.cpp
                                             constraining_templates02_cpp14.cpp
                                             constraining_templates03_cpp14.cpp
                                             constraining_templates04_cpp14.cpp)
