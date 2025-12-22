//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Gh9Y5srd7 
#include <iostream>
#include <future>  // async
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    auto f40 = std::async(fib, 40);
    auto f41 = std::async(fib, 41);
    auto f42 = std::async(fib, 42);
    auto f43 = std::async(fib, 43);
    /* ... at this point, further calculations can be made ... */
    cout << "fib(40): " << f40.get() << endl; // Output: fib(40): 102334155
    cout << "fib(41): " << f41.get() << endl; // Output: fib(41): 165580141
    cout << "fib(42): " << f42.get() << endl; // Output: fib(42): 267914296
    cout << "fib(43): " << f43.get() << endl; // Output: fib(43): 433494437
}