
// https://godbolt.org/z/rv5fWP5Ya
#include <random>
#include <vector>
#include <iostream>
using namespace std;
int main() {
    static const size_t size = 10;
    default_random_engine e{};              // Random number generator
    vector<size_t> counts(size+1);
    binomial_distribution<int> coins{size}; // tosses 10 coins, 0 to 10 heads
    for(auto i=120*1000; i>0; --i)
        ++counts[coins(e)];                 // Tossing coins
    for(auto c : counts) cout<<" "<<c;
    cout << '\n';
    // Example output:
    // 109 1159 5344 14043 24806 29505 24544 13973 5252 1150 115
}