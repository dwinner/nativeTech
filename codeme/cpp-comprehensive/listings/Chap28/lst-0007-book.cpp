
// https://godbolt.org/z/vdT7TTjWK 
#include <iostream>
#include <string>
#include <tuple>
#include <map>
#include <unordered_set>
using std::tuple; using std::tie; using std::cout; using std::string;
struct Point {
    int x,y,z;
    bool operator<(const Point &b) {
        return tie(x,y,z) < tie(b.x, b.y, b.z);
    }
};
int main() {
    Point a{ 11, 22, 33 };
    Point b{ 11, 33, 0 };
    cout << std::boolalpha << (a < b) << "\n"; // Output: true
}