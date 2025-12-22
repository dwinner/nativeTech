//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Md38b9zvf 
#include <chrono>
#include <iostream>
#include <format>
using namespace std::chrono;
int main() {
  auto ymd = last/February/2024;   // last day in February 2024: 2024-02-29
  std::cout << ymd << "\n";        // the output with << is simple
  std::cout << std::format("{:%Y-%m-%d}\n", ymd);  // format is more flexible
  std::cout << std::format("{:%e. %B %y}\n", ymd); // much more flexible!
}