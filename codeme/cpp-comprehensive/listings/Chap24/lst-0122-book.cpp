
// https://godbolt.org/z/5e3c88K8j
#include <bitset>
#include <iostream>
using std::cout;
int main() {
    std::bitset<8> bits{};         // 8 bits densely packed
    bits.set(4);                   // 5th bit to 1
    cout << bits << '\n';          // 00010000
    bits.flip();                   // invert all bits
    cout << bits << '\n';          // 11101111
    bits.set();                    // set all bits to 1
    bits.flip(1);                  // invert 2nd bit
    std::cout << bits << '\n';     // 11111101
    bits.reset();                  // set all bits to 0
    bits.set(4);                   // 5th bit to 1
    cout << bits << '\n';          // 00010000
    bits.flip();                   // invert all bits
    cout << bits << '\n';          // 11101111
    bits.set();                    // set all bits to 1
    bits.flip(1);                  // invert 2nd bit
    bits.flip(6);                  // invert 7th bit
    cout << bits << '\n';          // 10111101
    // Bitwise operations
    std::bitset<8> zack("....####", 8, '.', '#');
    cout << zack << '\n';          // 00001111
    cout << (bits & zack) << '\n'; // 00001101
    cout << (bits | zack) << '\n'; // 10111111
    cout << (bits ^ zack) << '\n'; // 10110010
    // other integer types
    std::bitset<64> b(0x123456789abcdef0LL);
    cout << b << '\n';
    // 0001001000110100010101100111100010011010101111001101111011110000
    cout << std::hex << b.to_ullong() << '\n'; // convert
    // 123456789abcdef0
}