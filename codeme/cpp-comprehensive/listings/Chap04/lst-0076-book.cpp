
// https://godbolt.org/z/6qfo4oYnr 
#include <iostream> 
#include <iomanip> // fixed, setprecision 
int main() { 
    using std::cout;
    cout << std::setprecision(30) << std::fixed; // always output 30 digits 
    cout << 1.111222333444555666777888999f << "\n"; // float literal 
    // output: 1.111222386360168457031250000000 
    cout << 1.111222333444555666777888999 << "\n";  // double is default 
    // Output: 1.111222333444555676607023997349 
    cout << 1.111222333444555666777888999d << "\n"; // double literal 
    // output: 1.111222333444555676607023997349 
    cout << 1.111222333444555666777888999L << "\n"; // long double 
    // output: 1.111222333444555666740784227731 
}