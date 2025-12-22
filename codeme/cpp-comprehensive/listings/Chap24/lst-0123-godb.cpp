//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KPP16bdK9 
#include <iostream>
#include <valarray>
using std::ostream; using std::valarray;
ostream& operator<<(ostream&os, const valarray<double>&vs) {
    os << "[";
    for(auto&v : vs) os << v << " ";
    return os << "]";
}
int main() {
    valarray a{ 1.0, 2.0, 3.0, 4.0 }; // valarray<double>
    valarray b{ 2.0, 4.0, 6.0, 8.0 };
    valarray c{ 2.5, 1.75, 0.5, 0.125 };
    valarray<double> x = ( a + b ) * c;
    std::cout << "x: " << x << "\n";  // Output: [7.5 10.5 4.5 1.5 ]
    auto y = ( a + b ) / 2;           // y is not necessarily a valarray!
    std::cout << "y: " << y << "\n";  // Output: [1.5 3 4.5 6 ]
}