//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xzY8hj9oP 
#include <chrono>
#include <future>
#include <mutex>
#include <vector>
#include <iostream>
std::timed_mutex mtx;
long fibX(long n) { return n < 2L ? 1L : fibX(n-1L) + fibX(n-2L); }
long fibCall(long n) {
    using namespace std::chrono; // Suffixes
    if(mtx.try_lock_for(1000ms)) {
        auto res = fibX(n);
        mtx.unlock();
        return res;
    } else {
        return 0L;
    }
}

int main() {
    std::vector< std::future<long> > fs;
    for(long n=1; n<= 42; ++n) {
        fs.emplace_back( std::async(std::launch::async, fibCall, n) );
    }
    for(auto &f : fs) {
        std::cout << f.get() << " ";
    }
    std::cout << std::endl;
}