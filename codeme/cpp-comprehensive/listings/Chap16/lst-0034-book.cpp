
// https://godbolt.org/z/vjrr6d5TW 
#include <iostream>               // istream, ostream, cout
class Bool {
    bool val_ = false;
    bool& operator*()             // dereference; mutable
        { return val_; };
    const bool& operator*() const // dereference; read-only
        { return val_; }
public:
    constexpr Bool() {}
    explicit constexpr Bool(bool value)
        : val_{value} {}
    // unary operators
    Bool operator!() const        // not operator
        { return Bool{!val_}; };
    // binary operators
    friend Bool operator&&(const Bool &re, const Bool &li)
        { return Bool{*re && *li}; }
    friend Bool operator||(const Bool &re, const Bool &li)
        { return Bool{*re || *li}; }
    // input and output
    friend std::ostream& operator<<(std::ostream& os, const Bool& arg);
    friend std::istream& operator>>(std::istream& is,  Bool& arg);
};
std::ostream& operator<<(std::ostream& os, const Bool& arg)
    { return os << *arg; }
std::istream& operator>>(std::istream& is, Bool& arg)
    { return is >> *arg; }
// Constants
static constexpr Bool False{false};
static constexpr Bool True{true};
int main() {
    Bool yesno = True && ( Bool{false} || !Bool{} ); // uses &&, ||, and !
    std::cout << yesno << "\n"; // Output: 1
}