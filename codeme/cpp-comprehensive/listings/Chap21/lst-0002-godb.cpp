//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/feGrM4f6j
// Filename: makros.cpp
#define OUTPUT_TO_STANDARD // Switch between cerr and cout
#include "my-macros.hpp"
#include "my-macros.hpp"  // Oops, accidentally included twice.
int main() {
    MESSAGE("Program start");
    container_type data(SIZE);
    MESSAGE("The container has " << data.size() << " elements.");
    MESSAGE("Program end");
    OUT << "That was a close call.\n";
}