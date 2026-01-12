
// https://godbolt.org/z/srczx737G
#include <vector>
#include <iostream>
#include <algorithm>               // ranges::sort
using std::vector; using std::cout;
double median(vector<int> data) {  // copied
    std::ranges::sort(data);       // C++20, otherwise std::sort()
    auto it = data.begin();
    auto sz = data.size();
    if(sz==0) return 0;            // special case
    // Determine median:
    auto m = (it+sz/2);            // approximately the middle
    if(sz%2 != 0) {                // odd number of elements
        return *m;
    } else {                       // even number of elements
        return double(*m + *(m-1)) / 2;
    }
}
int main() {
    vector data1 { 12, 22, 34, 10, 1, 99, 33 };
    cout << median(data1) << '\n'; // 22
    vector data2 { 30, 2, 80, 99, 31, 3 };
    cout << median(data2) << '\n'; // 30.5
}