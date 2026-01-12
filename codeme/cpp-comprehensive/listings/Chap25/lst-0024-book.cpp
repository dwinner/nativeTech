
// https://godbolt.org/z/dKPbjj1f8 
#include <iostream>
#include <memory>    // uninitialized_copy
#include <alloca.h>  // alloca (Linux)
#include <list>
int main () {
  const std::list input{1,9,2,6,6,6,8};
  const auto SZ = input.size();
  // uninitialized memory area:
  int* target = (int*)alloca(sizeof(int) * SZ); // space for 7 ints
  std::uninitialized_copy(input.begin(), input.end(), target);
  // test output
  for(int idx=0; idx<SZ; ++idx) {
    std::cout << target[idx] << ' ';
  }
  std::cout << '\n'; // output: 1 9 2 6 6 6 8
}