
// https://godbolt.org/z/7b8GaE9xT
#include <set>
#include <iostream>
using std::set; using std::cout;
template<typename Elem, typename Comp>
std::ostream& operator<<=(std::ostream&os, const set<Elem,Comp>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}