
// https://godbolt.org/z/fxoo4h9Gj 
#include <concepts>
#include <iostream>
#include <string>
using namespace std; using namespace std::literals;

string mk_string(integral auto val) { return to_string(val); }
string mk_string(string val) { return '['+val+']'; }
template<typename T>
  requires copyable<T> &&                            // Type requirement
  requires (T t) { mk_string(t) + mk_string(t); } && // simple requirement
  requires (T t) {                     // composite requirement
    {mk_string(t)} -> same_as<string>; // valid expression must meet condition
  }
string dbl_quote(const T& val) {
  T val2{val};                       // Create a copy (for demonstration purposes only)
  return '"' + mk_string(val) + mk_string(val2) + '"';
}
int main() {
  cout << dbl_quote(10) << '\n';    // Output: "1010"
  cout << dbl_quote("ab"s) << '\n'; // Output: "[ab][ab]"
  cout << dbl_quote(3.14) << '\n';  // (ERR) Error: no suitable mk_string overload
}