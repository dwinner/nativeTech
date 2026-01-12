//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jGx51caTj 
#include <vector>
#include <iostream>
void printAndMore(const std::vector<int>& data) { // by-const-ref
    std::cout << data[0] << std::endl;
    data[0] = 666;         // (ERR) doesn't work because 'const int&'
}
int main() {
    std::vector numbers {1,2,3};
    printAndMore(numbers);
}