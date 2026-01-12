//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7xYcqqh68 
#include <vector>
#include <iostream>   // cout
#include <iterator>   // ostream_iterator
#include <algorithm>  // copy

int main() {
  std::vector data { 1, 2, 3, 7, 9, 10 };
  std::ostream_iterator<int> out_it (std::cout,", "); // when assigned to cout
  std::copy(data.begin(), data.end(), out_it); // all elements to the iterator
  std::cout << "\n";                           // Output: 1, 2, 3, 7, 9, 10,
  // or from C++20 with Ranges:
  std::ranges::copy(data, out_it);
  std::cout << "\n";                           // Output: 1, 2, 3, 7, 9, 10,
}