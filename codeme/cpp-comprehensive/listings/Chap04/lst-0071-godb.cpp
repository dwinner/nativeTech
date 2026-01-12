//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jeTs5Exqx 
#include <iostream> 
int main() { 
    unsigned a = 0b1111'0000;        // 240 
    unsigned b = 0b0011'1100;        // 60 
    std::cout << ( a | b ) << "\n";  // Bit-Or: 252, in bits 1111'1100 
    std::cout << ( a & b ) << "\n";  // Bit-And: 48, in bits 0011'0000 
    std::cout << ( a ^ b ) << "\n";  // Exclusive-Or: 204, in bits 1100'1100 
    unsigned int c = 170;            // in bits 0..(24x0)..0'1010'1010 
    std::cout << ( ~c ) << "\n";     // Inv.: 4294967125, Bits: 1...(24x1)..1'0101'0101 
}