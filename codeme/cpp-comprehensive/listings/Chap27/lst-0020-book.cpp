
// https://godbolt.org/z/1xzzMdjdM 
#include <iostream>
using std::cout; using std::endl;
int main() {
    int val = 255;
    cout.setf(std::ios_base::hex, std::ios_base::basefield);
    cout.setf(std::ios_base::showbase);
    cout << val << std::endl; // Output: 0xff
    cout.unsetf(std::ios_base::showbase);
    cout.setf(std::ios_base::dec, std::ios_base::basefield);
    cout << val << std::endl; // Output: 255
}