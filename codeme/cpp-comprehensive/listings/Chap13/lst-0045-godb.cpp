//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eo468MvMo 
#include <iostream>
#include <string>
struct S {
   int n;
   std::string s;
   float d;
};
template <std::size_t N> auto& get(S& s) {
    if constexpr (N == 0) return s.n;
    else if constexpr (N == 1) return s.s;
    else if constexpr (N == 2) return s.d;
}
int main() {
    S obj { 0, "hello", 10.0f };
    std::cout << get<0>(obj) << ", " << get<1>(obj) << "\n”; // Output: 0, hello
}