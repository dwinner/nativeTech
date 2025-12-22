
// https://godbolt.org/z/jP75nY4EW
#include <concepts>  // integral
#include <iostream>
using namespace std;
// Concept explicitly with requires
template<typename T> requires integral<T>
auto add_1(T val) { return val+1; }
// abbreviated concept
template<integral T>
auto add_2(T val) { return val+2; }
// abbreviated function template with concept
auto add_3(integral auto val) { return val+3; }
// Ad-hoc requirements for function template
auto add_4(auto val) requires integral<decltype(val)>
{ return val+4; }
int main() {
  cout << add_1(1) << '\n';               // Output: 2
  cout << add_2(1) << '\n';               // Output: 3
  cout << add_3(1) << '\n';               // Output: 4
  cout << add_4(1) << '\n';               // Output: 5
  cout << add_3("text") << '\n';          // (ERR) Error
  integral auto val = add_1(99);          // also for auto variables
  cout << val << '\n';                    // Output: 100
}