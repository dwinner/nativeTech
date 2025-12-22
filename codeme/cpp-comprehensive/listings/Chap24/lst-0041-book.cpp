
// https://godbolt.org/z/h7jf4KM1q 
#include <vector>
#include <span>
#include <iostream>
using namespace std;
int sum(span<const int> area) {  // C++20: span
   int sum = 0;
   for(auto e : range) {         // algorithm would be better …
    sum += e;
   }
   return sum;
}
int main() {
    vector data {1,2,3,4,5};
    cout << sum(data) << "\n";   // implicit container to span
    auto [b, e, sz] = make_tuple(begin(data), end(data), size(data));
    cout << sum(span{b,   sz-1}) << "\n"; // 1..4 (Iter, size)
    cout << sum(span{b+1, sz-1}) << "\n"; // 2..5 (Iter, size)
    cout << sum(span{b,   e-2}) << "\n";  // 1..3 (Iter, Iter)
    cout << sum(span{b+1, e-1}) << "\n";  // 2..4 (Iter, Iter)
    cout << sum(span{b+2, e  }) << "\n";  // 3..5 (Iter, Iter)
    return 0;
}