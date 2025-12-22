//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5ddxa9v1j 
#include <iostream>                      // cout
int min3(int x, int y, int z) {          // function returns an int
    if(x<y) {
        if(x<z) return x;
        else return z;
    } else if(y<z) {
        return y;
    }
    else return z;
}
void printMin(int x, int y, int z) {    // function returns nothing
    if(x<0 || y<0 || z<0) {
        std::cout << "Please only numbers greater than 0\n";
        return;
    }
    std::cout << min3(x,y,z) << "\n";
}                                        // no return here
int main() {
    printMin(3, -4, 8);
    printMin(6, 77, 4);
    return;                              // special return in main
}