//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/r11584Wef 
#include <valarray>
#include <iostream>
using namespace std;
template<typename T>
ostream& operator<<=(ostream &os, const valarray<T>& a) { // '<<=' with newline
    for(const auto &v : a) os << v << ' ';
    return os << '\n';
}
int main() {
    // … example code here …
}