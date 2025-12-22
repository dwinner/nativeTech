//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xnbvEx5Kc 
#include <iostream>
#include <string>
using std::cout; using std::string; using namespace std::literals;
auto min2 = [](const auto &a, const auto &b) {
    return a<b ? a : b;
};
auto min3 = [](const auto &a, const auto &b, const auto &c) {
    return min2(a, min2(b,c));
};
int main() {
    cout << min3( 3, 7, 2 ) << '\n';                  // Output: 2
    cout << min3( 8.11, 113.2, -3.1 ) << '\n';        // Output: –3.1
    cout << min3( "Zoo"s, "Ape"s, "Mule"s ) << '\n';  // Output: Ape
}