
// https://godbolt.org/z/8ejPs6zr6 
#include <vector>
struct Number {
    int value_ = 0;
    Number() {} // Default constructor
    explicit Number(int v) : value_{v} {}
};
int main() {
    std::vector<Number> numbers{}; // okay: Number meets the requirements
    numbers.push_back( Number{2} );
}