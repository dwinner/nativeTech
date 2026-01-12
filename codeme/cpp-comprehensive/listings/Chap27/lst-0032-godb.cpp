//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/GefKP5sj4
#include <iostream>
#include <thread>
#include <syncstream>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long from, long step, long to) {
    for (auto n=from; n<=to; n+=step) {
        std::osyncstream osync{ std::cout }; // Sync on cout as long as osync exists
        osync << "fib("<<n<<")=" << fib(n) << '\n';
    }
}
int main() {
    std::jthread f40{ runFib, 1, 3, 40 };
    std::jthread f41{ runFib, 2, 3, 41 };
    std::jthread f42{ runFib, 3, 3, 42 };
}