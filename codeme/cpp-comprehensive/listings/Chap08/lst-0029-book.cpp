
// https://godbolt.org/z/185ha3q3f 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    int number = 1;                            // Initialization before the loop
    for(  ; number <= 100; number=number+1) {  // empty initialization
        sum = sum + number;
    }
    std::cout << number  << std::endl;         // number now also exists outside
}