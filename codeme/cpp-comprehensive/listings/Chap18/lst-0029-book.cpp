
// https://godbolt.org/z/Kvfx9Yeox 
#include <array>
class Value {
    int val_;
public:
    constexpr Value(int val) : val_{val} {};
    constexpr int get() const { return val_; }
};

constexpr Value five{5}; // works, Value is a literal data type

std::array<int,five.get()> data; // works, get is constexpr