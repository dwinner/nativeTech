
//https://godbolt.org/z/EcnWz3r3j 
#include <iostream>
#include <iomanip>                            // fixed, setprecision 
#include <format>                             // C++20 
using std::cout; using std::format;           // abbreviation cout, format 
int main() { 
    cout << std::fixed                        // dot notation, not scientific 
         << std::setprecision(15);            // 15 decimal places 
    cout << 0.5 << "\n";                      // output: 0.500000000000000* 
    cout << std::setprecision(5);             // 5 decimal places 
    cout << 0.25 << "\n";                     // output: 0.25000 
    cout << format("{:0.4f}", 0.75) << "\n";  // (C++20) output: 0.7500 
    return 0; 
}