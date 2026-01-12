//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/75h9Y95vf 
#include <iostream>                 // cout
int main() {
    for(int x=1; x<20; x+=1) {      // outer for-loop
        for(int y=1; y<20; y+=1) {  // inner for-loop
            int prod = x*y;
            if(prod>=100) {
                break;              // exit inner y-loop
            }
            std::cout << prod << " ";
        } /* end for y */
        // destination of break
    } /* end for x */               // first actual line after break
    std::cout << " ";
}