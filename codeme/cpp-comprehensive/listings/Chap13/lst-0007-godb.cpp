//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zfYzPjqrW
// modul.cpp
#include "modul.hpp"
static std::string PREFIX = "TREE:";
static void printInfo(std::ostream& os) {
    os << "Author: Torsten T. Will\n";
}
bool debug = false;
// rest as before