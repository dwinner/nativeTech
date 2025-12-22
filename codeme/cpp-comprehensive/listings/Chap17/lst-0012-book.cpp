
// https://godbolt.org/z/s8KoE4ETs 
#include <new> // nothrow
std::string *ps = new(std::nothrow) std::string{};
if(ps == nullptr) {
   std::cerr << "Memory allocation failed\n";
   return SOME_ERROR;
}