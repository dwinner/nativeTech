#include <iostream>
#include <format>
#include <chrono>
#include <vector>
#include <set>
#include <random>
#include <unordered_set>

// doesnt compile on on gcc 13.2
//#include <flat_set>
// using boost:
//   https://godbolt.org/z/cad38fvfK

using namespace std;
using namespace std::chrono;

constexpr long TOTAL = 100*1000*1000;
default_random_engine engine{};
mt19937 gen{engine()};
uniform_int_distribution<> randx(0, 1000);
auto roll = [] { return randx(gen); };

template<typename Container>
void random_test(const long xlimit, const char* name) {
    long total = 0;
    auto start = high_resolution_clock::now();
    while(total < TOTAL) {
        Container c{};
        for (int i = 0; i < xlimit; ++i, ++total) {
            c.insert(roll());
        }
    }
    auto stop = high_resolution_clock::now();
    auto duration = duration_cast<milliseconds>(stop - start);
    cout << format("{} -- Size: {}, Insert: {} ms\n", name, xlimit, duration.count());
}

int main() {
    vector<long> XLIMITS = {10, 100, 1000, 10*1000, 100*1000, 1000*1000, 10*1000*1000}; // 100*1000*1000};
    for(const auto xlimit : XLIMITS) {
        random_test<set<int>>(xlimit, "set<int>");
        random_test<unordered_set<int>>(xlimit, "unordered_set<int>");
//        random_test<flat_set<int>>(xlimit, "flat_set<int>");
    }
}


