//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s1csa51z9 
#include <iostream>
#include <iomanip>
using std::cin; using std::cout; using std::endl;

void f(bool b) {
     cout << b << endl;                     // Output:  true
}

int main () {
    bool b=true;
    cout << std::boolalpha << b << endl;   // Output: true
    b=false;
    cout << b << endl;                     // Output: false
    f(true);
    cout << std::noboolalpha << b << endl; // Output: 0
    b=true;
    cout << b << endl;                     // Output: 1
}