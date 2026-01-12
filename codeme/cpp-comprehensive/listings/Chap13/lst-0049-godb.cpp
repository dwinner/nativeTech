//#(compile) c++; compiler:g132; options:"-std=c++23"; libs:-
// https://godbolt.org/z/7KWqTPo1K 
#include <iostream> // cout
constexpr int get_value() {
    if consteval {
       return 42;
    } else {
       return 668;
    }
}
int main() {
  auto a = get_value();
  std::cout << a << '\n';        // Output: 668
  constexpr auto b = get_value();
  std::cout << b << '\n';        // Output: 42
}