
// https://godbolt.org/z/o9oYrvMcj 
#include <iostream>
#include <future>  // async
#include <chrono>
using std::cout; using std::endl; using namespace std::chrono;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
int main() {
    auto f43 = std::async(fib, 43);
    while(true) {
        auto ready = f43.wait_for(500ms);
        if(ready==std::future_status::timeout) {
            std::cout << "not yet..." << std::endl;
        } else {
            break;
        }
    }
    // pick up, is immediately there
    cout << "fib(43): " << f43.get() << endl; // Output: fib(43): 701408733
}