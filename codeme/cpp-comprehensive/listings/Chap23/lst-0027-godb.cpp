//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/r83Yeon7r
#include <iostream>                            // cout
#include <limits>                              // numeric_limits
template<typename INT_TYP>                     // Template with type argument
void infos(const char* name) {
    using L = typename std::numeric_limits<INT_TYP>; // rename for brevity
    std::cout
        << name
        << " number of bits:" << L::digits     // Bits without sign bit
        << " sign:" << L::is_signed            // stores sign?
        << " min:"<< (long long)L::min()       // smallest possible value
        << " max:"<< (long long)L::max()       // largest possible value
        << "\n";
}
int main() {
    infos<signed char>("char");                // smallest int type
    infos<short>("short");
    infos<int>("int");
    infos<long>("long");
    infos<long long>("long long");             // largest int type
}