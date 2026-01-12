//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KvsGh4f1v 
namespace lits {
  // Template helper function for one argument
  template<char C> int bin() {  // general case
    if constexpr (C=='0') return 0;
    else if constexpr (C=='1') return 1;
  }
  // Template helper function for two or more arguments
  // …
}