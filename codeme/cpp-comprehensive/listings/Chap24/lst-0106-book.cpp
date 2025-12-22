
// https://godbolt.org/z/38TbjGhn6
#include <unordered_set>
#include <iostream>
using std::unordered_set; using std::cout; using std::ostream;
template<typename Elem, typename Cmp>
ostream& operator<<=(ostream&os, const unordered_set<Elem,Cmp>&data){
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}