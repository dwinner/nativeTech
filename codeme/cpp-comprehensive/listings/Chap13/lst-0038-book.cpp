
// https://godbolt.org/z/xPzcWPdMa 
#include <array>
struct Data {
    static const size_t LATE;             // declare constant
    static const size_t EARLY;            // declare constant
};

void func() {
    int x = Data::LATE;                   // use constant
}

const size_t Data::EARLY = 10;            // define constant

std::array<int, Data::EARLY> arrEARLY {}; // use constant
std::array<int, Data::LATE>  arrLATE {};  // (ERR) use constant
const size_t Data::LATE = 10;             // define constant

int main() {
    func();
}