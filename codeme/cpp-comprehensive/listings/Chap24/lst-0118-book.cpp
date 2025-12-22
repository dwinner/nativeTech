
// https://godbolt.org/z/bvehE8hse
#include <unordered_set> // unordered_multiset
#include <vector>
#include <iostream>
using std::unordered_multiset; using std::cout; using std::ostream;
template<typename Elem>
ostream& operator<<=(ostream&os, const unordered_multiset<Elem>&data) {
    for(auto &e : data) { os << e << ' '; } return os << '\n'; }
int main() {
    // without arguments
    unordered_multiset<int> empty(1000); // initial size of the hash table
    cout <<= empty;    // Output:
    // Initialization list; duplicates are included:
    unordered_multiset data{ 1,1,2,2,3,3,4,4,5,5 };
    cout <<= data;     // Output approximately: 5 5 4 4 3 3 2 2 1 1
    // Copy
    unordered_multiset copi(data);
    cout <<= copi;     // Output approximately: 5 5 4 4 3 3 2 2 1 1
    // Range
    std::vector in{1,2,3,10,20,30,10,20,30,1,2,3};
    unordered_multiset rang(in.begin()+3, in.end()-3);
    cout <<= rang;     // Output approximately: 30 30 20 20 10 10
}