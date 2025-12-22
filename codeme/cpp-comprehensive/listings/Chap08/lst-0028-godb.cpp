//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7foYcMMaz 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    for(int number=1; number <= 100; number+=1)  {  // compact form
        sum += number;                              // for the result
    }
    std::cout << sum << std::endl;
}