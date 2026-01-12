
// https://godbolt.org/z/z6sYqM48e 
#include <iostream>
#include <string>
void show(double f) {
    std::cout << "os: " << f
       << "\t to_string: " << std::to_string(f) << "\n";
}
int main() {
    show(23.43);     // Output: os: 23.43    to_string: 23.430000
    show(1e-9);      // Output: os: 1e-09    to_string: 0.000000
    show(1e40);      // Output: os: 1e+40    to_string: 100…0752.000000
    show(1e-40);     // Output: os: 1e-40    to_string: 0.000000
    show(123456789); // Output: os: 1.23457e+08  to_string: 123456789.000000
}