//#(compile) c++; compiler:gsnapshot; options:"-std=c++23"; libs:-
// https://godbolt.org/z/Gfazq6GE7 
#include <generator>
#include <iostream>
#include <vector>

std::generator<int> fib(int n) { // generates int values
  int a = 0, b = 1;
  while (--n) {
    co_yield b;                  // makes this function a coroutine
    auto tmp = a;
    a = b;
    b += tmp;
  }
}
int main() {
  for (auto i : fib(10)) std::cout << i << ' ';
  std::cout << '\n';              // Output: 1 1 2 3 5 8 13 21 34 55
}