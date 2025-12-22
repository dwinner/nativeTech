//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Wbr7n8M4q 
#include <iostream>
#include <array>
constexpr bool isPrime(int n) { // computable at compile-time
  if(n < 2) return false; // 0, 1 are not prime
  for (int i = 2; i*i <= n; i += i>2 ? 2 : 1) { // 2,3,5,7,9,11,13,15…
    if (n % i == 0) return false;
  }
  return n > 1; // for 0 and 1
}
template<int Num>
consteval std::array<int, Num> primeNumbers() { // only at compile-time
  std::array<int, Num> primes{};
  int idx = 0;
  for (int val = 1; idx < Num; ++val) {
    if (isPrime(val)) primes[idx++] = val;
  }
  return primes;
}
int main() {
  // initialize with prime numbers
  auto primes = primeNumbers<100>();  // 1000000 doesn't work
  for (auto v : primes) {
    std::cout << v << ' ';
  }
  std::cout << '\n';
}