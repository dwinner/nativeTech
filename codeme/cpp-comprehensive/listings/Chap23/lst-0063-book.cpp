
// https://godbolt.org/z/Kxrz5ETh6 
#include <concepts>  // integral, floating_point
#include <iostream>
using namespace std;
int64_t trunc_to_10(integral auto val) { return (val/10)*10; }
int64_t trunc_to_10(floating_point auto val) { return int64_t(val/10)*10; }
int main() {
  cout << trunc_to_10(144) << '\n';    // Output: 140
  cout << trunc_to_10(122.2) << '\n';  // Output: 120
}