//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/T4d4jdqox
#include <vector>
#include <iostream>
#include <algorithm>
using std::vector; using std::ostream; using std::cout;
int main() {
    vector<int> data{};
    data.reserve(400);                     // Space for 400 elements
    // Phase 1: fill
    for(int idx = 1; idx <= 20; ++idx) {
        for(int val = 0; val < 20; ++val) {
            data.push_back(val % idx);    // something between 0 and 19
        }
    }
    cout << data.size() << '\n';          // 400 elements between 0 and 19
    // Post-processing Phase 1: create set-equivalent
    std::sort(data.begin(), data.end());  // preparation for unique
    auto wo = std::unique(data.begin(), data.end()); // duplicates to the end
    data.erase(wo, data.end());           // remove duplicates
    data.shrink_to_fit();
    cout << data.size() << '\n';          // only 20 elements left
    // Phase 2: use
    for(auto &e:data)
        cout << e << ' ';                 // Output: 0 1 2 .. 18 19
    cout << '\n';
    auto it = std::lower_bound(data.begin(), data.end(), 16); // search value
    if(it!=data.end() && *it == 16)
        cout << "found!\n";
    if(std::binary_search(data.begin(), data.end(), 7))   // yes or no
        cout << "also found!\n";
}