cmake_minimum_required(VERSION 3.28)
project("prefer_task_based")
set(CMAKE_CXX_STANDARD 23)

add_executable(software_threads software_threads.cpp)
add_executable(task_based_approach task_based_approach.cpp)
add_executable(thread_based_approach thread_based_approach.cpp)

target_link_libraries(software_threads ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(task_based_approach ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(thread_based_approach ${CMAKE_THREAD_LIBS_INIT})
