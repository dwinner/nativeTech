#include <iostream>
#include <string>
using std::string;

namespace lits {
    // helper. Note: This is not efficient, using string+string recursivly.
    template<char C> string sx() { return string{}+C; }
    template<char C, char D, char... ES>
    string sx() {
        return sx<C>() + sx<D,ES...>();
    }
    // operator"" _sx
    template<char...CS> string operator"" _sx()
    { return sx<CS...>(); };
}
int main() {
    using namespace lits;
    std::cout << 10000000_sx << '\n';
    //= 10000000
    std::cout << 10'000'000_sx << '\n';
    //= 10'000'000
    std::cout << 0x00af_sx << '\n';
    //= 0x00af
    std::cout << 0x0'c'0'a'f_sx << '\n';
    //= 0x0'c'0'a'f
    std::cout << 007_sx << '\n';
    //= 007
    std::cout << 0b01_sx << '\n';
    //= 0b01
    //std::cout << 0b0a8sh3s1_sx << '\n';
    //std::cout << "abcde"_sx << '\n';
}

