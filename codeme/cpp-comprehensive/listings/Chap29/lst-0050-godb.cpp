//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/rYM9xqsGa
#include <future> // async
#include <vector>
#include <algorithm> // max
#include <iostream>

int calculateHeight(int count, int maxCount, int scale) {
  if(maxCount == 0)
      throw std::logic_error("All heights 0");
  return (count * scale) / maxCount;
}

void bar(const std::vector<int> &counts) {
  // Start calculation
  auto maxCount = *std::max_element(counts.begin(), counts.end());
  std::vector< std::future<int> > futs;
  for(int count : counts) {
    futs.push_back(
          std::async(std::launch::async,
              calculateHeight, count, maxCount, 200) );
  }

  // Collect results
  for(auto &fut : futs) {
    std::cout << fut.get() << ' ';                 // triggers exception
  }
  std::cout << '\n';
}

int main() {
  try {
    bar(std::vector {10,23,13,0,33,4 });         // Output: 60 139 78 0 200 24
    bar(std::vector { 0, 0, 0, 0 });             // triggers exception
  } catch(std::exception &ex) {
    std::cout << "Error: " << ex.what() << '\n'; // Output: Error: All heights 0
  }
}