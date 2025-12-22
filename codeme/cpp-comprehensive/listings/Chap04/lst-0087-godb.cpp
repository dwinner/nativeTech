//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/43rYYeKx9 
#include <iostream> 
#include <iomanip> // setprecision, fixed
#include <complex> 
using std::cout; using std::complex;

int main() {
    using namespace std::complex_literals;   // for i-suffix 
    cout << std::fixed << std::setprecision(1); 
    complex<double> z1 = 1i * 1i;            // i times i 
    cout << z1 << '\n';                      // output: (-1.0,0.0)
    complex<double> z2 = std::pow(1i, 2);    // i-square 
    cout << z2 << '\n';                      // Output: (-1.0,0.0) 
    double PI = std::acos(-1);               // Length of half a unit circle 
    complex<double> z3 = std::exp(1i * PI);  // Euler formula 
    cout << z3 << '\n';                      // Output: (-1.0,0.0) 
    complex<double> a(3, 4);                 // usually as a constructor 
    complex<double> b = 1. - 2i;             // practically as a literal

    // Calculations: 
    cout << "a + b = " << a + b << "\n";     // Output: a + b = (4.0,2.0) 
    cout << "a * b = " << a * b << "\n";     // Output: a * b = (11.0,-2.0) 
    cout << "a / b = " << a / b << "\n";     // Output: a / b = (-1.0,2.0) 
    cout << "|a| = " << abs(a) << "\n";      // Output: |a| = 5.0 
    cout << "conj(a) = " << conj(a) << "\n"; // Output: conj(a) = (3.0,-4.0) 
    cout << "norm(a) = " << norm(a) << "\n"; // Output: norm(a) = 25.0 
    cout << "abs(a) = " << abs(a) << "\n";   // Output: abs(a) = 5.0 
    cout << "exp(a) = " << exp(a) << "\n";   // Output: exp(a) = (-13.1,-15.2) 
}