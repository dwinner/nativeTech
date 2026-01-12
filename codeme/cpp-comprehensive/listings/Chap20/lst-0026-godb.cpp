//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/a5b6qMo9h
#include <iostream>
int main() {
    int carray[10] = { 1,1 };             // initialized to { 1,1,0,0,0,0,0,0,0,0 }
    int* end = carray+10;                 // pointer past the last element
    for(int* p =carray+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);             // adds the previous two numbers
    }
    for(int const * p=carray; p != end; ++p)
        std::cout << *p << " ";
    std::cout << "\n";
}