
// https://godbolt.org/z/7r4b8jdGr 
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
using std::to_string; using std::string; using std::vector;
struct Squares {
    mutable int n = 1;
    int operator()() const { return n*n++; }
};
int main() {
    vector<int> sq(10);
    std::ranges::generate(sq, Squares{});
    std::ranges::for_each(sq, [](auto n) {
        std::cout << n << " "; });
    std::cout << '\n';         // Output: 1 4 9 16 25 36 49 64 81 100
    string a = "NCC-";
    vector<int> b {1,7,0,1};
    vector<string> c(4);
    auto f = [](char c, int i) -> string { return c+to_string(i); };
    std::ranges::transform(
        a,           // Input 1
        b,           // Input 2
        c.begin(),   // Output
        f);          // string f(char,int)
    std::ranges::for_each(c, [](auto s) {
        std::cout << s << " "; });
    std::cout << '\n';         // Output: N1 C7 C0 -1
}