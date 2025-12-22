
// https://godbolt.org/z/e67356bMq
#include <random>
#include <chrono>  // system_clock
#include <format>
#include <iostream>

using namespace std;
using namespace chrono;
const size_t LOOPS = 10*1000*1000;

template<typename ENGINE>
void measure(const string &title, ENGINE &engine) {
  const auto start = steady_clock::now();

  /* work hard */
  unsigned long long checksum = 0;
  size_t loops = LOOPS;
  while(loops-- > 0)
    checksum += engine();

  const auto now = steady_clock::now();
  nanoseconds dur_ns = now - start;
  cout << std::format("  {}: {:5} ns/loop  {:12} ns\n",
      title, dur_ns.count() / LOOPS, dur_ns.count());
}

int main() {
  { default_random_engine e{}; measure("       default", e ); }
  { random_device e{};         measure("        device", e ); }
  { minstd_rand0 e{};          measure("  minstd_rand0", e ); }
  { minstd_rand e{};           measure("  minstd_rand ", e ); }
  { mt19937 e{};               measure("    mt19937   ", e ); }
  { mt19937_64 e{};            measure("    mt19937_64", e ); }
  { ranlux24_base e{};         measure(" ranlux24_base", e ); }
  { ranlux48_base e{};         measure(" ranlux48_base", e ); }
  { ranlux24 e{};              measure(" ranlux24     ", e ); }
  { ranlux48 e{};              measure(" ranlux48     ", e ); }
  { knuth_b e{};               measure("       knuth_b", e ); }
  {
    using wide_t = unsigned long long ;
    independent_bits_engine<ranlux48, sizeof(wide_t)*8, wide_t> e{};
    measure("indep<ranlux> ", e );
  }
  {
    using wide_t = unsigned long long;
    independent_bits_engine<
          default_random_engine,
          sizeof(wide_t)*8, wide_t>
      e {};
    measure("indep<default>", e );
  }
}