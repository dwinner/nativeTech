
// https://godbolt.org/z/Mevoshb3o 
namespace lits {
  long double operator"" _w(long double);
  string      operator"" _w(const char16_t*, size_t);
  unsigned    operator"" _w(const char*);
}
int main() {
  using namespace lits;
  1.2_w;       // operator"" _w(long double), with (1.2L)
  u"one"_w;    // operator"" _w(char16_t, size_t), with (u"one", 3)
  12_w;        // operator"" _w(const char*), with "12"
  "two"_w;     // (ERR) operator"" _w(const char*, size_t) not defined
}