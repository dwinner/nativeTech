//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/K834js588
#include <map>        // the main thing
#include <iostream>   // for output
#include <string>     // often used for key or target
using std::map; using std::cout; using std::string;
template<typename Key, typename Trg, typename Comp>
std::ostream& operator<<=(std::ostream&os, const map<Key,Trg,Comp>&data) {
    for(auto &e : data) os << e.first << ":" << e.second << ' ';
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}