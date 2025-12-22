//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cPoEjqzM7 
#include <iostream>
#include <string>
namespace lits {
    // operator"" _sx
    template<char...CS> std::string operator"" _sx() {
        return (std::string{} + ... + CS); // a fold expression
    };
}
int main() {
    using namespace lits;
    std::cout << 10000000_sx << '\n';     // Output: 10000000
    std::cout << 10'000'000_sx << '\n';   // Output: 10'000'000
    std::cout << 0x00af_sx << '\n';       // Output: 0x00af
    std::cout << 0x0'c'0'a'f_sx << '\n';  // Output: 0x0'c'0'a'f
    std::cout << 007_sx << '\n';          // Output: 007
    std::cout << 0b01_sx << '\n';         // Output: 0b01
}