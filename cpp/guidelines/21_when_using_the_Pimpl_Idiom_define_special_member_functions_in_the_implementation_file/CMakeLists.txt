cmake_minimum_required(VERSION 3.28)
project("pimpl")
set(CMAKE_CXX_STANDARD 23)

#add_subdirectory(no_pimpl)
add_subdirectory(pimpl_raw_ptr)
#TODO: fix linker error!!!  add_subdirectory(pimpl_unique_ptr)
add_subdirectory(pimpl_shared_ptr)
