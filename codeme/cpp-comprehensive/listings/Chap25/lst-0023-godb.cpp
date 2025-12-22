//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/enj3znvTs
#include <numeric> // *_scan
#include <iostream>
#include <vector>
using std::inclusive_scan; using std::exclusive_scan;
std::ostream& operator<<=(std::ostream&os, const std::vector<int>&data) {
    for(auto &e : data) os << e << ' '; return os << '\n';
}
int main() {
  std::vector data{ 1, 3, 10, 18, 30, 50 };
  std::vector<int> result(6);    // 6 elements
  auto plus = [](auto a, auto b) { return a+b; };
  inclusive_scan(data.begin(),data.end(), result.begin(), plus, 100);
  std::cout <<= result;
  // Output: 101 104 114 132 162 212
  exclusive_scan(data.begin(),data.end(), result.begin(), 100);
  std::cout <<= result;
  // Output: 100 101 104 114 132 162
}