//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/YzYz3x1zx 
#include <iostream>
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    int x = 0;
    int y = 0;
    int z = MAX2( ++x, ++y );  // (ERR) expands arguments multiple times
    std::cout << "x:"<< x << " y:"<< y << " z:"<<z << '\n';
}