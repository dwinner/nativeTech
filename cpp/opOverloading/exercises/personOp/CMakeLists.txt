cmake_minimum_required(VERSION 3.29)
project(personOp)

set(CMAKE_CXX_STANDARD 23)

add_executable(personOp main.cpp
        Person.cpp
        Person.hpp)
