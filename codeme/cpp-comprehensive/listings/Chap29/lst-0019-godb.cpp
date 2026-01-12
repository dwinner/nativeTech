//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/zojG68dWq
#include <thread>
#include <iostream>
#include <vector>
#include <chrono>  // steady_clock
using std::cout; using std::endl; using namespace std::chrono;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
  cout << std::thread::hardware_concurrency() << '\n';
  for(int nthreads : { 1,2,3,4,5,6 }) {
    cout << "Threads: ";
    const auto start = steady_clock::now();

    std::vector<std::jthread> threads;
    for(int ti = 1; ti <= nthreads; ++ti) {
      threads.emplace_back( std::jthread{fib, 40});
      cout << ti << "... "; cout.flush();
    }
    for(auto &th : threads) th.join(); // explicitly join before timing

    const auto now = steady_clock::now();
    cout << "  Time:  " << duration_cast<milliseconds>(
        now-start).count()<<"ms\n";
  }
}