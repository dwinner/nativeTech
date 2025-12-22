//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/P9MjGe6Mn 
#include <iostream>
#include <future>  // async
#include <vector>
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
int main() {
    // Prepare tasks
    std::vector< std::future<long> > fibs;
    for(int n=0; n<50; ++n) {
        auto fut = std::async(std::launch::deferred, fib, n);
        fibs.push_back( std::move(fut) );
    }
    // only retrieve the required result
    cout << "fib(42): " << fibs[42].get() << endl; // Output: fib(42): 267914296
}