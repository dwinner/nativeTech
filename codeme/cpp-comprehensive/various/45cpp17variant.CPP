#include <variant>
#include <iostream>
using std::get;
int main() {
    std::variant<int, float> v{};
    v = 12;                      // State changes to int
    auto i = get<int>(v);        // get the int
    std::cout << i << '\n';      // (+Output:+) 12
    //= 12
    v = 3.456f;                  // State changes to float
    std::cout << get<float>(v) << '\n';  // (+Output:+) 3.456
    //= 3.456

    std::variant<int, float> w{};
    w = get<float>(v);           // Access via type
    w = get<1>(v);               // Access via index also works
    w = v;                       // whole assignment also works
    try {
        get<int>(w);             // throws exception
    } catch (std::bad_variant_access&) {
        std::cout << "currently does not hold an int\n";
        //= currently does not hold a float
    }
}