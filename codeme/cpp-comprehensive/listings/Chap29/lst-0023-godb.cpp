//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rrfn4d5WP 
#include <thread>
#include <latch>
#include <iostream>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    std::latch la{ 3 };                             // we expect 3 threads
    std::jthread th1{ [&la] { fib(39); la.count_down(); } };
    std::jthread th2{ [&la] { fib(38); la.count_down(); } };
    std::jthread th3{ [&la] { fib(40); la.count_down(); } };
    fib(37); // main thread
    std::cout << "Main thread: done\n";
    la.wait();                                      // waits until la == 0
    std::cout << "Rest done\n";
}