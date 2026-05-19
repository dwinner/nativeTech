cmake_minimum_required(VERSION 4.1)
project(mylib C)

set(CMAKE_C_STANDARD 23)

add_library(mylib SHARED library.c)
