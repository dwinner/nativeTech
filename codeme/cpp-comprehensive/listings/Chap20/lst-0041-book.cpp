
// https://godbolt.org/z/zsqKz76cj 
#include <iostream>   // cout
#include <iterator>   // ostream_iterator
#include <algorithm>  // copy, ranges::copy
int main () {
  int data[6] = { 1, 2, 3, 7, 9, 10 };
  std::ostream_iterator<int> out_it (std::cout,", ");
  std::copy(data, data+6, out_it);            // Pointers as iterators
  std::cout << "\n";                          // Output: 1, 2, 3, 7, 9, 10
  std::ranges::copy(data, out_it);            // C-array pointer as range
  std::cout << "\n";                          // Output: 1, 2, 3, 7, 9, 10
}