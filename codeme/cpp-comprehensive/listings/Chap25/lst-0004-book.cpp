
// https://godbolt.org/z/Koxoha5bx
#include <algorithm>  // find
#include <numeric>    // reduce, accumulate
#include <execution>  // std::execution
#include <iostream>
#include <chrono>     // time measurement
using namespace std::chrono;
long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}
template <typename FUNC> void timeit(const char* title, FUNC func) {
    auto start = steady_clock::now();
    auto ret = func(); // execute
    std::cout << title << ": " << millisSince(start) << " ms" << std::endl;
}
int main() {
    using namespace std::execution;          // seq, par, par_unseq
    using std::reduce; using std::accumulate; using std::find;
    std::vector<double> v(600'000'000, 0.0); // 600 million elements
    for(auto&x:v) x = ::rand();              // fill with random values
    timeit("warm-up       ", [&v] {
        return reduce(seq, v.begin(), v.end(), 0.0);
    });
    timeit("accumulate       ", [&v] {
        return accumulate(v.begin(), v.end(), 0.0);
    });
    timeit("reduce, seq      ", [&v] {
        return reduce(seq, v.begin(), v.end(), 0.0);
    });
    timeit("reduce, par      ", [&v] {
        return reduce(par, v.begin(), v.end(), 0.0);
    });

    timeit("reduce, par_unseq", [&v] {
        return reduce(par_unseq, v.begin(), v.end(), 0.0);
    });
    timeit("find, seq        ", [&v] {
        return find(seq, v.begin(), v.end(), 1.1) == v.end() ? 0.0 : 1.0;
    });
    timeit("find, par        ", [&v] {
        return find(par, v.begin(), v.end(), 1.1) == v.end() ? 0.0 : 1.0;
    });
    return 0;
}