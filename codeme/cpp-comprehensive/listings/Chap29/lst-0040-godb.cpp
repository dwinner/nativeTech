//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/c9x6xT8dr
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <deque>
#include <iostream>
#include <syncstream>   // osyncstream
std::deque<int> g_data{};          // Data exchange between threads
std::condition_variable g_condvar; // notify
std::mutex g_mx;                   // protects g_data during changes
void produce(int limit) {
  std::vector primes{2};           // prior primes as test-divisors
  for(int candidate=3; candidate < limit; candidate+=2) {
    for(int divisor : primes) {
      if(divisor*divisor > candidate) {     // candidate is prime
        std::lock_guard lk{g_mx};           // protect data
        g_data.push_back(candidate);        // fill
        g_condvar.notify_one();             // notify
        primes.push_back(candidate);        // for internal calculations
        break; // next prime candidate
      } else if(candidate % divisor == 0) { // not prime
        break;                              // next prime candidate
      } else {
        // next divisor to check
      }
    }
  }
  // notify all work done
  std::lock_guard lk{g_mx};            // protect data
  g_data.push_back(0);                 // fill with end marker
  g_condvar.notify_all();              // notify
}
void consume(char l, char r) {
  while(true) {                        // forever
    std::unique_lock lk{g_mx};
    g_condvar.wait(lk, []{ return !g_data.empty();});
    int prim = g_data.front();         // fetch data
    if(prim == 0) return;              // done; leave 0 for other consumers
    g_data.pop_front();
    lk.unlock();                       // release lock
    std::osyncstream osync{std::cout}; // synchronize output
    osync << l << prim << r <<' ';
  }
}
int main() {
  // a producer:
  std::jthread thProd{produce, 1'000};
  // three consumers
  std::jthread thCon1{consume, '[', ']' };
  std::jthread thCon2{consume, '<', '>' };
  std::jthread thCon3{consume, '{', '}' };
  // wait and finish
  thProd.join();
  thCon1.join(); thCon2.join(); thCon3.join();
  std::cout << '\n';
}