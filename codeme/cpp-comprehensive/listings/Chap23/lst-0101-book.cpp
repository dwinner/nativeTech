
// https://godbolt.org/z/sbxojqjqs 
namespace lits {
  // Template helper function for one argument
  template<char C> int bin();  // general case
  template<>       int bin<'1'>() { return 1; } // Specialization
  template<>       int bin<'0'>() { return 0; } // Specialization
  // Template helper function for two or more arguments
  template<char C, char D, char... ES>
  int bin() {
    return bin<C>() << (sizeof...(ES)+1) | bin<D,ES...>(); // Bit-Shift, Bit-Or
  }
  // actual operator""
  template<char...CS> int operator"" _bin()
    { return bin<CS...>(); };
}
int main() {
  using namespace lits;
  int one = 1_bin;
  int eight = 1000_bin;
  int nine = 1001_bin;
  int ten = 1010_bin;
  int eleven  = 1011_bin;
  int one_hundred_twenty_eight = 10000000_bin;
}