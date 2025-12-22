
// https://godbolt.org/z/5Whz59374
#include <numeric>    // adjacent_difference
#include <functional> // plus
#include <algorithm>  // copy
#include <iostream>
#include <iterator>   // ostream_iterator
#include <vector>
#include <ranges>     // pairwise_transform
using std::cout; using std::vector; namespace vs = std::views;
int main() {
    // Stream output iterator for int;
    std::ostream_iterator<int> os{std::cout, " "};
    vector data{ 1, -1, 2, -2, -4, 4, -6, 6 };
    std::copy(data.begin(), data.end (), os);
    cout << '\n'; // Output: 1 -1 2 -2 -4 4 -6 6
    vector<int> res( data.size()-1 ); // Space for result
    // Write results to res:
    adjacent_difference(data.begin(), data.end(), res.begin());
    std::copy (res.begin(), res.end (), os);
    cout << '\n'; // Output: 1 -2 3 -4 -2 8 -10
    // Write directly to os:
    adjacent_difference(data.begin(), data.end(), os, std::plus<int>{});
    cout << '\n'; // Output: 1 0 1 0 -6 0 -2 0
    // or via range adapter:
    for(auto e: vs::pairwise_transform(data, std::plus<int>{}))
        cout << e << ' ';
    cout << '\n'; // Output: 0 1 0 –6 0 –2 0
}