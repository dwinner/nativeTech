//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/YonbGKj8M 
#include <iostream>  // istream, ostream, cout
class Num {
  int val_ = 0;
public:
  int& operator*();  // Dereference: Direct access to the value
  const int& operator*() const; // Dereference: Read access to the value
  Num() {}
  explicit Num(int value) : val_{value} {}
  // unary operators
  Num& operator++(); // Preincrement
  Num& operator--(); // Predecrement
  Num operator+();   // Positive
  Num operator-();   // Negate
  Num operator~();   // Bitwise invert
  // binary operators
  // - compound assignments, arithmetic
  Num& operator+=(const Num& re) { val_ += *re; return *this; }
  Num& operator-=(const Num& re) { val_ -= *re; return *this; }
  Num& operator*=(const Num& re) { val_ *= *re; return *this; }
  Num& operator/=(const Num& re) { val_ /= *re; return *this; }
  Num& operator%=(const Num& re) { val_ %= *re; return *this; }
  // - compound assignments, bitwise
  Num& operator|=(const Num& re) { val_ |= *re; return *this; }
  Num& operator&=(const Num& re) { val_ &= *re; return *this; }
  Num& operator^=(const Num& re) { val_ ^= *re; return *this; }
  Num& operator<<=(int n) { val_ <<= n; return *this; }
  Num& operator>>=(int n) { val_ >>= n; return *this; }
  // - Variation of compound assignments for easier handling
  Num& operator+=(int re) { val_ += re; return *this; }
  Num& operator-=(int re) { val_ -= re; return *this; }
  // binary operators, with call-by-value for the first parameter
  // and using compound assignment for assistance
  // - Arithmetic
  friend Num operator+(Num li, const Num& re) { return li += re; }
  friend Num operator-(Num li, const Num& re) { return li -= re; }
  friend Num operator*(Num li, const Num& re) { return li *= re; }
  friend Num operator/(Num li, const Num& re) { return li /= re; }
  friend Num operator%(Num li, const Num& re) { return li %= re; }
  // - bitwise
  friend Num operator|(Num li, const Num& re) { return li |= re; }
  friend Num operator&(Num li, const Num& re) { return li &= re; }
  friend Num operator^(Num li, const Num& re) { return li ^= re; }
  // - comparisons
  // - … fundamental for standard containers and algorithms
  friend bool operator==(const Num& li, const Num& re) { return *li == *re; }
  auto operator<=>(const Num& re) const {return val_ <=> *re; } // C++20
  // - input and output
  friend std::ostream& operator<<(std::ostream& os, const Num& arg);
  friend std::istream& operator>>(std::istream& is,  Num& arg);
};
// unary operators
Num& Num::operator++() { ++val_; return *this; }
Num& Num::operator--() { --val_; return *this; }
Num Num::operator+() { return Num{val_}; }
Num Num::operator-() { return Num{-val_}; }
Num Num::operator~() { return Num{~val_}; }
int& Num::operator*() { return val_; }
const int& Num::operator*() const { return val_; }
// Input and Output
std::ostream& operator<<(std::ostream&os, const Num& arg) { return os<<*arg; }
std::istream& operator>>(std::istream&is, Num& arg) { return is>>*arg; }