
// https://godbolt.org/z/Yd4sser5j 
#include <complex>
// raw form
int operator"" _dual(const char*);
int answer = 101010_dual;    // decimal 42
// preprocessed form
std::complex<long double> operator"" _i(long double d) {
  return std::complex<long double>(0, d);
}
auto val = 3.14_i; // val = std::complex<long double>(0, 3.14)