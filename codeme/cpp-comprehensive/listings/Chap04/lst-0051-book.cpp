
// https://godbolt.org/z/qdY41d5a9
#include <iostream> 
#include <bitset> 
constexpr unsigned n_bits = sizeof(unsigned short)*8; // 8 bits per char 
auto reverse_bits(unsigned val) -> unsigned short { 
  unsigned short ret = 0; 
  for (unsigned i = 0; i < n_bits; ++i ) { 
    ret = (ret << 1) | (val & 1); // one to the side, set lowest bit
    val >>= 1;                    // one in the other direction 
  } 
  return ret; 
} 
void tell(unsigned short val) { 
  std::bitset<n_bits> bits{val}; 
  std::cout << val << "=" << bits << " -> "; 
  auto lav = reverse_bits(val); 
  std::bitset<n_bits> stib{lav}; 
  std::cout << lav << "=" << stib << "\n"; 
} 
int main() { 
  tell(36u);  // output: 36=0000000000100100 -> 9216=0010010000000000 
  tell(199u); // output: 199=0000000011000111 -> 58112=1110001100000000
  tell(255u); // Output: 255=0000000011111111 -> 65280=1111111100000000 
  tell(256u); // Output: 256=0000000100000000 -> 128=0000000010000000 
}