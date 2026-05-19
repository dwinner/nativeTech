cmake_minimum_required(VERSION 4.1)
project(factorial C)

set(CMAKE_C_STANDARD 23)

add_library(factorial STATIC library.c)
