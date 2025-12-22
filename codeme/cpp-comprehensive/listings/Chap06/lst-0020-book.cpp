
// https://godbolt.org/z/xcKM7v1Eo 
#include <vector> 
#include <iostream>                          // cout, endl 
int main() { 
  std::vector qus{1,4,9,16,25}; 
  for(unsigned idx=0; idx<qus.size(); ++idx) // size returns the count 
    std::cout << qus[idx] << " ";            // [idx] or at(idx) gets an element
  std::cout << std::endl; 
}