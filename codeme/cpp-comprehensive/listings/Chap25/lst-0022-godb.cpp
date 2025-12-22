//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/WPrP79sv6
#include <numeric>   // accumulate, iota
#include <algorithm> // copy
#include <iostream>
#include <iterator>  // ostream_iterator
#include <vector>
#include <ranges>    // iota, take, stride
using std::accumulate; using std::cout; using std::vector; 
namespace vs = std::views;
struct Generator {
    int state_;
    void operator++() { state_ += state_; }
    operator int() { return state_; }
};
int main() {
    std::ostream_iterator<int> os{std::cout, " "}; // Stream output iterator for int
    vector<int> data(7);
    std::iota(data.begin(), data.end(), 10);
    std::copy(data.begin(), data.end (), os);
    cout << '\n';           // Output: 10 11 12 13 14 15 16
    vector<int> seq(7);
    std::iota(seq.begin(), seq.end(), Generator{2});
    std::copy(seq.begin(), seq.end(), os);
    cout << '\n';           // Output: 2 4 8 16 32 64 128
    for(auto i: vs::iota(0) | vs::stride(3) | vs::take(6))
        cout << i << ' ';
    cout << '\n';           // Output: 0 3 6 9 12 15
}