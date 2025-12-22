
// https://godbolt.org/z/q4ov41cnv
#include <iostream>
double twice(double num) {     // value parameter and return value 
    return num * 2.0; 
} 
int main() { 
    double num = 7.25;
    num = twice(num);          // change expressed by return value 
    std::cout << num << "\n";  // also 14.5 
}