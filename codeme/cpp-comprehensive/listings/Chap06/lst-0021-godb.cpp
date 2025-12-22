//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1WG8zvhYe 
#include <vector> 
#include <iostream>                               // cout, endl 
int main() { 
  std::vector qus{1,4,9,16,25}; 
  for(auto it = qus.begin(); it!=qus.end(); ++it) // between begin() and end() 
    std::cout << *it << " ";  // with *it you get from the iterator to the element 
  std::cout << std::endl; 
}