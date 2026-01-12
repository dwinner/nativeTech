//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/1MYhxb7rq
#include <set>               // set, multiset
#include <unordered_set>     // unordered_set, unordered_multiset
#include <iostream>
#include <string>
#include <chrono>            // Time measurement
using std::cout;
using namespace std::chrono;

long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}
constexpr size_t ITERATIONS = 100'000;
template<typename Cont, typename Gen>
    requires std::invocable<Gen, size_t> &&  // C++20 concept
    requires(Gen gen, size_t n) {{gen(n)} -> std::same_as<int>;} &&
    std::same_as<typename Cont::value_type,int>
void timeStuff(std::string name, Cont data, Gen genNum) {
    cout << name << "...";
    auto start = steady_clock::now();
    for(size_t idx=0; idx<ITERATIONS; ++idx) {
        data.insert( genNum(idx) );
    }
    cout << " " << millisSince(start) << " ms" << std::endl;
}
int same(size_t) { return 7; }      // always generates the same number
int scatter(size_t n) { return int(n); } // generates different numbers
struct BadHash {      // the worst possible hash function as a functor
       size_t   operator()(int) const { return 1uz; }
};

int main() {
   std::multiset<int> m{};
   timeStuff("multiset           same           ", m, &same);
   timeStuff("multiset           scatter        ", m, &scatter);
   std::set<int> s{};
   timeStuff("set                same           ", s, &same);
   timeStuff("set                scatter        ", s, &scatter);
   std::unordered_multiset<int> um{};
   timeStuff("unordered_multiset same           ", um, &same);
   timeStuff("unordered_multiset scatter        ", um, &scatter);
   std::unordered_multiset<int,BadHash> umb{};
   timeStuff("unordered_multiset same    badHash", umb, &same);
   timeStuff("unordered_multiset scatter badHash", umb, &scatter);
}