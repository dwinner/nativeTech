cmake_minimum_required(VERSION 3.28)
project("varying_thread_hdl")
set(CMAKE_CXX_STANDARD 23)

add_library(item38_example1 example1.cpp)
target_link_libraries(item38_example1 pthread)

add_executable(item38_example2 example2.cpp)
target_link_libraries(item38_example2 pthread)
