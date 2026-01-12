//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/j6WWxPheP
// Filename: my-macros.hpp
#ifndef MY_MACROS_HPP  // Include Guard
#define MY_MACROS_HPP
#include <iostream>    // cout, cerr
#include <vector>
#ifdef OUTPUT_TO_STANDARD
#  define OUT std::cout
#else
#  define OUT std::cerr
#endif
#define MESSAGE(text) { (OUT) << text << "\n"; }
using container_type = std::vector<int>;
static constexpr unsigned SIZE = 10;
#endif