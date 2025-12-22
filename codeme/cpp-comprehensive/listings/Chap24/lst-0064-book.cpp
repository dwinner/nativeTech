
// https://godbolt.org/z/so5Gebb4M 
#include <list>
#include <iostream>
using std::list; using std::cout; using std::ostream;

ostream& operator<<=(ostream&os, const list<int> &data)
    { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main() {
    list numa { 1, 3, 5, 7, 9 };
    list numb { 2, 4, 6, 8 };
    auto wh = numa.end();
    numa.splice(wh, numb); // transfer in O(1)
    cout <<= numa; // Output: 1 3 5 7 9 2 4 6 8
    cout <<= numb; // Output: (none)
    numa.sort();   // sort as a method, not from <algorithm>
    cout <<= numa; // Output: 1 2 3 4 5 6 7 8 9
}