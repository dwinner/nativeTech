//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/PrM4fGcWo 
#include <array>
#include <iostream>
using std::array; using std::cout;
template<typename Elem, size_t SZ>
std::ostream& operator<<(std::ostream&os, const array<Elem,SZ>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os;
}
int main() {
   // Example code here
}