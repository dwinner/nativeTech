
// https://godbolt.org/z/5hqanTeYh 
#include <iostream> 
#include <bitset>  // helps with the output of numbers as a bit sequence 
int main() { 
    int a = 0; 
    for(int idx=0; idx<8; idx++) { 
        a <<= 2;                         // shift two bits to the left: "...100"
        a |= 1;                          // set the lowest bit: "...1"
    }
    std::cout << std::bitset<16>(a) << "\n"; // 0101010101010101 
    std::cout << a << "\n";              // 21845 
}