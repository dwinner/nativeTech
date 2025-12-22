//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5rdzn8YP4 
namespace lits {
  // general template
  template<char...> int operator"" _bin2();
  // specializations
  template<> int operator"" _bin2<'0','0'>() { return 0; }
  template<> int operator"" _bin2<'0','1'>() { return 1; }
  template<> int operator"" _bin2<'1','0'>() { return 2; }
  template<> int operator"" _bin2<'1','1'>() { return 3; }
}
int main() {
  using namespace lits;
  int one   = 01_bin2;
  int three = 11_bin2;
}