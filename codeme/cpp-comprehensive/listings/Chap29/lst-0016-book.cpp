
// https://godbolt.org/z/YrEPnYMaP 
#include <iostream>
#include <thread>
#include <vector>
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long n) { auto r = fib(n); cout << "fib("<<n<<")=" << r << endl; }
int main() {
    std::vector<std::jthread> threads;
    // starten
    for( auto n : { 38, 39, 40, 41, 42, 43, }) {
        threads.emplace_back( runFib, n );
    }
}