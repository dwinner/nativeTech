
// https://godbolt.org/z/6xzTx83zE 
#include <string>
#include <cmath> // sin, cos
constexpr double max2(double a, double b) { return  a > b ? a : b; }
constexpr int max2(int a, int b) { return  a > b ? a : b; }
std::string max2(const std::string &a, const std::string &b)
    { return  a > b ? a : b; }
int main() {
    double e = max2(sin(3.141592/2), cos(3.141592/2));
    double i = max2(10+12+45, 100/5+20);
    std::string s = max2("Ernie", "Bert");
}