
// https://godbolt.org/z/oTn1K4sGG 
#include <vector>
#include <iostream>  // cout, ostream
#include <algorithm> // sort, copy
#include <iterator>  // ostream_iterator
int main() {
    std::ostream_iterator<int> oit(std::cout," ");
    std::vector data { 100, 50, 1, 75, 25, 0 };       // vector<int>
    std::sort(std::begin(data), std::end(data));      // Iterator pair
    std::copy(std::begin(data), std::end(data), oit); // Iterator pair
    std::cout << '\n';                                // Output: 0 1 25 50 75 100
    std::ranges::reverse(data);                       // Range
    std::ranges::copy(data, oit);                     // Range
    std::cout << '\n';                                // Output: 100 75 50 25 1 0
}