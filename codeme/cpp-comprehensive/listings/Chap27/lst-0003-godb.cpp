//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/WaEGdnWvq
#include <iostream> // cin, cout
int main() {
    int val1, val2;
    std::cout << "Please enter 2 int values: ";
    std::cin >> val1 >> val2;
    std::cout << val1 << " : " << val2 << std::endl;
}