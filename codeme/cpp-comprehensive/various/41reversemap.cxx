#include <map>
#include <iostream>
#include <algorithm> // reverse

int MY_MARKER = 123456789; //#

std::ostream& operator<<=(std::ostream&os, const std::map<int,int>& cont) {
    for(auto &e : cont) {
        os << e.first << ":" << e.second << " ";
    }
    return os;
}

int main() {
    // arrange
    std::map<int,int> m{};
    m[3] = 300;
    m[7] = 700;
    m[2] = 200;
    m[8] = 800;
    m[1] = 100;
    m[5] = 500;
    m[6] = 600;
    std::cout << "Before: " <<= m;
    // act
#if 0 // I am relieved that this doesnt work:
    std::reverse(m.begin(), m.end());
#endif
    // assert
    std::cout << "After: " <<= m;
}
