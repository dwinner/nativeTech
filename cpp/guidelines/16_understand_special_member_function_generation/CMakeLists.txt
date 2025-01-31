cmake_minimum_required(VERSION 3.28)
project("special_member_functions")
set(CMAKE_CXX_STANDARD 23)

add_library(itemXX Widget01.cpp
                   Widget02.cpp
                   Widget03.cpp
                   StringTable01.cpp
                   StringTable02.cpp)
