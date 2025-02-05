cmake_minimum_required(VERSION 3.28)
project("unjoinable_threads")
set(CMAKE_CXX_STANDARD 23)

add_executable(example1 example1.cpp)
target_link_libraries(example1 pthread)

add_executable(example2 example2.cpp)
target_link_libraries(example2 pthread)
