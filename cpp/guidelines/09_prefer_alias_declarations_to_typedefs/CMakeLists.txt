cmake_minimum_required(VERSION 3.28)
project("prefer_using_alias")
set(CMAKE_CXX_STANDARD 23)

add_library(item09 function_pointers.cpp
                   linked_list_synonym_with_alias_template01.cpp
                   linked_list_synonym_with_alias_template02.cpp
                   linked_list_synonym_with_typedef01.cpp
                   linked_list_synonym_with_typedef02.cpp
                   # TODO wine.cpp
                   )
