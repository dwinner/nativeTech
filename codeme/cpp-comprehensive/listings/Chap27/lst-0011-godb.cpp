//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bEKaYdssE 
#include <iostream>
#include <ios>
using std::cout; using std::endl;
void f() {
    int val = 100;
    cout << val << endl;                // Output: 0x64
}
int main() {
    int val = 255;
    cout << std::showbase;
    cout << std::dec << val << endl;   // Output: 255
    cout << std::hex << val << endl;   // Output: 0xff
    f();
    cout << std::oct << val << endl;   // Output: 0377
    cout << val << std::endl;          // Output: 0377
}