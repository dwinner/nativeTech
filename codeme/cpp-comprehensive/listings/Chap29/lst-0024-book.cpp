
// https://godbolt.org/z/515dMo9Kx
#include <thread>
#include <barrier>
#include <iostream>
#include <vector>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

constexpr int anz = 8;          // 8 workers
constexpr int max_n = 32;       // calculate up to 32
std::vector<long> results(anz); // Buffer: space for 8 results

void output() {                 // Signaling function prints buffer
    for (auto n : results) std::cout << n << ' ';
    std::cout << '\n';
}

std::barrier ba{anz, output};   // always output after 8

void worker(std::stop_token st, int idx) {
    // n = 0, 9, 17, 25, … ; 1, 10, 18, 26, …
    for(int n = idx; n<max_n; n += anz) {
        if(st.stop_requested()) return;
        results[idx] = fib(n);  // write result to buffer
        ba.arrive_and_wait();   // wait until 8 threads are here
    }
}

int main() {
    std::vector<std::jthread> threads;     // 8 Threads
    for (int idx=0; idx<anz; ++idx) {
        threads.emplace_back(worker, idx); // create thread with index
    }
    for (auto& t : threads) t.join();      // wait until all are done
}