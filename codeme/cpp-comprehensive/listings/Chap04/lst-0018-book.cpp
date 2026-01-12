
// https://godbolt.org/z/r5hjrqqK9 
#include <iostream> 
int main() { 
    int var = 10; 
    var += 2; 
    var *= 3; 
    var /= 4; 
    var -= 5; 
    std::cout << var << "\n"; // results in 4 
}