//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y1W8WfxYn 
#include <array>
void calculate(const std::array<int,4>& data) { /* ... */ }
void calculate(const std::array<int,5>& data) { /* ... */ }