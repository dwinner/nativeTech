
// https://godbolt.org/z/MW74f97f7
#include <set>
#include <iostream>
using std::cout; using std::ostream; using std::set;
void search(const set<int>&data, int what, ostream&os) {
    os << what << "? ";
    // contains
    auto inside = data.contains(what);  // C++20
    os << "inside:" << (inside ? "yes." : "no."); // check contains
    auto where = data.find(what);
    if(where != data.end()) {
        os << " found:" << *where << " ";
    } else {
        os << " not found. ";
    }
    auto lo = data.lower_bound(what);
    if(lo != data.end()) {
        os << "lo:" << *lo;
    } else {
        os << "lo:-";
    }
    auto up = data.upper_bound(what);
    if(up != data.end()) {
        os << " up:" << *up;
    } else {
        os << " up:-";
    }
    // [lo,up] is now the same as what equal_range would have returned
    os << " Range:{";
    for( ; lo != up; ++ lo) {
      os << *lo << ' ';
    }
    os << "}";
    // count
    os << " C:" << data.count(what) // count hits
       << "\n";
}
int main() {
    set data{ 10, 20, 30, 40, 50, 60 };
    search(data, 20, cout); // 20? in:yes. found:20 lo:20 up:30 range:{20 } C:1
    search(data, 25, cout); // 25? in:no. lo:30 up:30 range:{} C:0
    search(data, 10, cout); // 10? in:yes. found:10 lo:10 up:20 range:{10 } C:1
    search(data, 60, cout); // 60? in:yes. found:60 lo:60 up:- range:{60 } C:1
    search(data, 5, cout);  // 5? in: no. lo:10 up:10 Range:{} C:0
    search(data, 99, cout); // 99? in: no. lo:- up:- Range:{} C:0
}