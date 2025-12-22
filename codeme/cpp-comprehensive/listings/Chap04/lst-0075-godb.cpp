//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qx8h7a4j9
#include <iostream> 
#include <iomanip>                                 // fixed, setprecision 
using std::cout;                                   // abbreviated cout 
int main() { 
    cout << std::setprecision(2) << std::fixed;    // two decimal places 
   cout << "1/4: " << 0.25 << "\n";                // comma notation for double 
    // output: 1/4: 0.25

    cout << "2/4: " << 0.5 << "\n"; 
    // Output: 2/4: 0.50 
    cout << "3/4: " << 0.75 << "\n"; 
    // Output: 3/4: 0.75 
    cout << "4/4: " << 1 << " or " << 1.0 << "\n"; // (ERR) recognizes 1 as int 
    // Output 4/4: 1 or 1.00 
    cout << "1e0: " << 1e0 << "\n";                // scientific notation 
    // output: 1e0: 1.00 
    cout << "0x10.1p0: " << 0x10.1p0 << "\n";      // hexadecimal notation 
    // output: 0x10.1p0: 16.06 
}