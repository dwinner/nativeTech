//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h3baE65od 
#include <iostream>
#include <vector>
#include <set>
#include <bitset>
#include <ranges>
namespace rng = std::ranges;
std::ostream& printBinary(std::ostream& os, rng::input_range auto&& range) {
    for(auto&& elem : range) {
        std::bitset<4> x(elem); // Copy number into bitset
        os << x << " ";
    }
    return os;
}
int main()
{
    std::vector vdata { 2, 0, 15, 12 };
    printBinary(std::cout, vdata) << "\n";
    // Output: 0010 0000 1111 1100
    std::set sdata { 2, 0, 12, 15 };
    printBinary(std::cout, sdata) << "\n";
    // Output: 0000 0010 1100 1111
    int adata[] = { 0,1,2,13,14,15 };
    printBinary(std::cout, adata) << "\n";
    // Output: 0000 0001 0010 1101 1110 1111
}