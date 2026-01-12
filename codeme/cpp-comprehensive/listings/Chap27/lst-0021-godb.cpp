//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4MY1Yznv9 
#include <iostream>
using std::cout; using std::endl;
int main() {
    int val = 255;
    std::ios::fmtflags ff = std::cout.flags();
    cout.flags(std::ios::hex | std::ios::showbase);
    cout << val << endl;  // Output: 0xff
    cout.flags(ff);
    cout << val << endl;  // Output: 255
}