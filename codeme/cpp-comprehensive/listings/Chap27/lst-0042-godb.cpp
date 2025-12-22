//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/hTj94enjK
#include <format>
#include <chrono>
#include <string>
#include <string_view>
#include <iostream>
using namespace std; using namespace std::literals;
void pr(string_view s) { cout << s << endl; }
double pi = 3.14159265359;

int main() {
  pr(format("Hello, {}!", "Reader"));              // simple C-string
  pr(format("Hello, {}!", "Author"s));             // simple string
  pr(format("You are {} years old.", 30));         // integers
  pr(format("That makes {:.2f} euros.", 19.9933)); // 2 decimal places
  pr(format("Scientific: {:e}", -44.876)); // results in "-4.487600e+01"
  pr(format("Binary of {} is {:b}.", 42, 42));     // binary without base
  pr(format("Hex of {} is {:#x}.", 73, 73));       // hexadecimal with base
  pr(format("Zero-padded: {:03}", 7));     // results in "007"
  pr(format("|{0:<10}|{1:^10}|{2:>10}|", "le", "mi", "ri"));
  // Alignment and index
  pr(format("{} {:.9}!", "Boa", "Constrictor")); // without index, truncate string
  using namespace std::chrono;                   // neat time specifications:
  pr(format("{}, {}", 2023y/11/5, minutes{20})); // Output: 2023-11-05, 20min
}