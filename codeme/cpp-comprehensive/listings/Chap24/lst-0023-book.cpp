
// https://godbolt.org/z/WK86eaz1x 
#include <vector>
#include <iostream>
using std::vector; using std::cout;
template<typename T>
std::ostream& operator<<(std::ostream&os, const vector<T>& data) {
    for(const auto &e : data) {
        os << e << ' ';
    }
    return os;
}
int main() {h
   // Example code here
}