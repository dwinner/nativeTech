//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/3MGbYn4d5
#include <iostream>
#include <chrono>
long fib(long n) { return n<2L ? 1L : fib(n-1L)+fib(n-2L); }
int main() {
    using namespace std::chrono;
    auto t0 = steady_clock::now();  // On your marks, get set …
    auto res = fib(17);             // … go!
    auto t1 = steady_clock::now();  // Stop!
    std::cout << "Result: " << res << "\n"; // Output: Result: 2584
    std::cout << "Time: " << nanoseconds{t1-t0}.count() << "ns\n";
    // Output: Time: 50727ns (e.g.)
}