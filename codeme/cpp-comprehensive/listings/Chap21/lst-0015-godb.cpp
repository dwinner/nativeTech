//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jzKfabPh3 
#include <cmath> // sin, cos
constexpr double max2(double a, double b) { return a > b ? a : b; }
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    double f = max2(sin(3.141592/2), cos(3.141592/2));
    double e = MAX2(sin(3.141592/2), cos(3.141592/2));
}