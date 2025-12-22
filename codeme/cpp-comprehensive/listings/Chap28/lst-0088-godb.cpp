//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/baY7Tz91f
#include <variant> 
using std::get;
int main() {
    std::variant<int, float> v{};
    v = 12;                   // State changes to int
    auto i = get<int>(v);     // retrieves the int
    std::cout << i << '\n';   // Output: 12
    v = 3.456f;               // State changes to float
    std::cout << get<float>(v) << '\n';  // Output: 3.456
    get<double>(v);           // (ERR) Error
    get<3>(v);                // (ERR) Error
    std::variant<int, float> w{};
    w = get<float>(v);        // Access by type
    w = get<1>(v);            // Access is also possible via index
    w = v;                    // entire assignment is also possible
    try {
        get<int>(w);          // triggers exception
    } catch (std::bad_variant_access&) { /* ... */ }
}