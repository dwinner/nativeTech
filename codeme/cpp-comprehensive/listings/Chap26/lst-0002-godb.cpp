//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/fGKsfs1Y4 
#include <iostream>  // cout
#include <algorithm> // copy
#include <iterator>  // ostream_iterator
#include <vector>
int main() {
  std::vector<char> path{};
  for (char ch = 'a'; ch <= 'z'; ++ch) {
    path.push_back(ch);
  }
  std::ranges::copy(path, // here everything, but it also works with other ranges
    std::ostream_iterator<char>(std::cout, " ") // copy to cout, separator " "
  );
}