
// https://godbolt.org/z/acMjvxcM5 
#include <string>
#include <cmath> // sin, cos
template<typename TYPE>
constexpr TYPE max2(const TYPE &a, const TYPE &b)
    { return  a > b ? a : b; }
int main() {
    double e = max2(sin(3.141592/2), cos(3.141592/2));
    double i = max2(10+12+45, 100/5+20);
    std::string s = max2(std::string("Ernie"), std::string("Bert"));
}