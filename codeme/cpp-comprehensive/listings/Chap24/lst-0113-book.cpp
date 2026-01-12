
// https://godbolt.org/z/T5KKWx4z4 
#include <unordered_map>
#include <iostream>
using std::unordered_map; using std::cout;
template<typename K, typename T>
std::ostream& operator<<=(std::ostream&os, const unordered_map<K,T>&data) {
    for(auto &e : data) {
        os << e.first << ":" << e.second << ' ';
    }
    return os << '\n'; // with operator<<= followed by a newline
}
int main() {
   // Example code here
}