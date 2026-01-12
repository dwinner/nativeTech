
// https://godbolt.org/z/aMj87neG6
#include <set>     // multiset
#include <iostream>
using std::multiset; using std::cout;
template<typename Elem, typename Comp>
std::ostream& operator<<=(std::ostream&os, const multiset<Elem,Comp>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}