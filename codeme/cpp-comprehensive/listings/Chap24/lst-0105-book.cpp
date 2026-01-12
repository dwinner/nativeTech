
// https://godbolt.org/z/cc3KohPrY
#include <unordered_set>     // unordered_set, unordered_multiset
#include <iostream>
#include <string>
#include <chrono>            // Time measurement
using std::cout;
using namespace std::chrono;
long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start)
        .count();
}
struct BadHash {  // the worst possible hash function as a functor
    size_t   operator()(int) const { return 1uz; }
};
void timeStuff(size_t iters) {
    std::unordered_multiset<int,BadHash> data{};
    cout << iters << "...";
    auto start = steady_clock::now();
    for(size_t idx=0; idx<iters; ++idx) {
        data.insert( (int)idx );
    }
    cout << " " << millisSince(start) << " ms" << std::endl;
}
constexpr size_t LIMIT = 20'000;
int main() {
    size_t iters = 100;
    while(iters < LIMIT) {
        timeStuff(iters);
        iters *= 2; // double
    }
}