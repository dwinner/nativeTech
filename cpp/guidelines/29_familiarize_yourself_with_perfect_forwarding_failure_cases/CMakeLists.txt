cmake_minimum_required(VERSION 3.28)
project("perfect_forwarding")
set(CMAKE_CXX_STANDARD 23)

add_executable(braced_initializers
               braced_initializers.cpp)
add_executable(null_pointers
               null_pointers.cpp)
add_executable(declaration-only_integral_static_const_data_members
               declaration-only_integral_static_const_data_members.cpp)
add_executable(overloaded_function_names_and_template_names
               overloaded_function_names_and_template_names.cpp)
add_executable(bitfields
               bitfields.cpp)
