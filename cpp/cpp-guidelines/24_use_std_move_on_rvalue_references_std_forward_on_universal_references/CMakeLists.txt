cmake_minimum_required(VERSION 3.28)
project("rvalue_and_forward")
set(CMAKE_CXX_STANDARD 23)

add_executable(dont_use_move_with_urefs dont_use_move_with_urefs.cpp)
add_executable(dont_use_move_with_urefs_solution dont_use_move_with_urefs_solution.cpp)

add_library(item25 rrefs_are_moveable.cpp
                   urefs_maybe_moveable.cpp
# TODO             move_or_forward_only_on_final_use.cpp
                   reduce_and_copy.cpp
                   matrix.cpp
                   return_value_optimization.cpp
                   )
