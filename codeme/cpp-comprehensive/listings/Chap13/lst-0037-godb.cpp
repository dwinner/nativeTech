//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/E6MWxY1f7
#include <array>
int main() {

    std::array<int, 5> arr5{};     // literal and thus a constant expression
    std::array<int, 2+3> arr23{};  // 2+3 can be evaluated by the compiler

    const size_t SIZE = 5;         // defines a constant
    std::array<int, SIZE> arrSC{}; // can often be used by the compiler

    size_t size = 7;
    std::array<int,size> arrVar{}; // you cannot use a variable
}