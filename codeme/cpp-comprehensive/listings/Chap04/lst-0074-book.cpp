
// https://godbolt.org/z/8ae83nPYs
#include <iostream>     // cout 
#include <iomanip>      // setprecision, etc.
#include <cmath>        // fabs 
using std::cout;        // cout as abbreviation for std::cout 
int main() { 
    cout << std::fixed << std::setprecision(25);     // for better readable output 
    // 0.1 and 0.01 cannot store double exactly
    double x = 0.1 * 0.1; 
    cout << "0.1*0.1: " << x << "\n"; 
    // output: 0.1*0.1: 0.0100000000000000019428903 
    if(x == 0.01) {     // (ERR) never compare double with == 
        cout << "Yes! x == 0.01" << "\n"; 
    } else {
        cout << "Uh-oh! x != 0.01" << "\n";          // you see this output 
    } 
    // Attention especially when comparing with 0.0
    double null = x - 0.01; 
    cout << "null: " << null << "\n"; 
    // Output: null: 0.0000000000000000017347235 
    if(std::fabs(null) < 0.00000001) {               // compare with an "epsilon"
        cout << "Yes! null is close to 0.0" << "\n"; // you see this output 
    } else { 
        cout << "Uh-oh! null not close to 0.0" << "\n"; 
    } 
    // fractions of powers of 2 are less critical 
    double y = 0.5 * 0.5; 
    cout << "0.5*0.5: " << y << "\n"; 
    // Output: 0.5*0.5: 0.2500000000000000000000000 
    if(y == 0.25) {     // here the dangerous comparison works exceptionally
        cout << "Yes! y == 0.25" << "\n";            // you see this output 
    } else { 
        cout << "Oh-oh! y != 0.25" << "\n"; 
    } 
    // 
    return 0; 
}