cmake_minimum_required(VERSION 3.28)
project("overloading_unirefs")
set(CMAKE_CXX_STANDARD 23)

add_executable(logAndAdd_inefficient logAndAdd_inefficient.cpp)
add_executable(logAndAdd_with_uref logAndAdd_with_uref.cpp)
add_executable(logAndAdd_with_uref_overloaded logAndAdd_with_uref_overloaded.cpp)
add_executable(Person Person.cpp)
