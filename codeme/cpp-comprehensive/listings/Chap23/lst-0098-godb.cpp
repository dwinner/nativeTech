//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cjx49ef19 
namespace my {
  Complex operator"" _i(const char*); // 0+ni
  Complex operator"" _j(long double); // n+0i
  Puzzle operator"" _puzzle(const char*, size_t);
}
using namespace my;
Complex imag4 = 4_i;      // operator"" _i(const char*)
Complex real3 = 3.331_j;  // operator"" _j(long double)

Puzzle p1 = "oXo"         // operator"" _puzzle(const char*, size_t)
            "XoX"_puzzle;