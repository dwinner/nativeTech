
// https://godbolt.org/z/6Ez5GdozG 
#include <vector>
#include <span>
#include <iostream>
using namespace std;
void inc(span<int> span) {
   for(auto& e :span) {  // Reference
     e += 1;             // write
   }
}
int main() {
    vector data {1,2,3,4,5};
    span whole{data};              //    1,2,3,4,5
    inc(whole.first(3));           // -> 2,3,4,4,5
    inc(whole.last(3));            // -> 2,3,5,5,6
    inc(whole.last(4).first(3));   // -> 2,4,6,6,6
    inc(whole.subspan(1,3));       // -> 2,5,7,7,6
    for(auto i: whole) cout << i << ' '; cout << '\n'; // Output: 2 5 7 7 6
    return 0;
}