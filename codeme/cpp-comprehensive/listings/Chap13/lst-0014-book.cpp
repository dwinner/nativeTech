
// https://godbolt.org/z/115M1csEE 
#include <iostream>
namespace mylib {
    namespace v1 {
        int version() { return 1; }
    }
    inline namespace v2 { // current version
        int version() { return 2; }
    }
}

int main() {
    std::cout << "Version " << mylib::version() << "\n”;     // Output: 2
    std::cout << "Version " << mylib::v1::version() << "\n”; // Output: 1
    std::cout << "Version " << mylib::v2::version() << "\n”; // Output: 2
}