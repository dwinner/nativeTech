//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h466fYq4W 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    int number = 1;
    while(number <= 100)      // condition
    {                         // block that is repeatedly executed
        sum += number;        // for the result
        number += 1;          // next number
    }                         // end of the repeated block
    std::cout << sum << std::endl;
}