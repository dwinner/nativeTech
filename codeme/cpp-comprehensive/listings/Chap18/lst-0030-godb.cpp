//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/r1vYc7r5j 
#include <array>
#include <iostream>
#include <type_traits> // is_literal_type
class Value {
    int val_;
public:
    constexpr Value(int val) : val_{val} {};
    constexpr operator int() const { return val_; }
};
namespace lit {
    constexpr Value operator"" _val(const char*, size_t sz) { 
      return Value(sz); }
}
struct Nope {
    constexpr Nope() {};
    virtual ~Nope() {};                 // non-constexpr destructor
};
int main() {
    using namespace lit;
    constexpr Value five{5};
    std::array<int,"11111"_val> data;  // use user-defined literal
    std::cout << data.size() << '\n';                        // Output: 5
    std::cout << std::boolalpha;
    std::cout << std::is_literal_type<Value>::value << '\n'; // Output: true
    std::cout << std::is_literal_type<Nope>::value << '\n';  // Output: false
}