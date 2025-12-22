//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s8KoE4ETs 
#include <new> // nothrow
std::string *ps = new(std::nothrow) std::string{};
if(ps == nullptr) {
   std::cerr << "Memory allocation failed\n";
   return SOME_ERROR;
}