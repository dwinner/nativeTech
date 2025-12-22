
// https://godbolt.org/z/h6nrPYWfM 
#include <array>
#include <iostream>
int main() {
    std::array ports { 80, 443  };          // array<int>
    auto [ http, https ] = ports;           // declares variables and binds them
    std::cout << http << " " << https << "\n";
    auto const& [ rhttp, rhttps ] = ports;  // Reference and const are possible
    std::cout << rhttp << " " << rhttps << "\n";
}