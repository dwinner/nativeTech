//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yhzqo6dxT
#include <set>
struct Number {
    int value_ = 0;
    explicit Number(int v) : value_{v} {}
};
bool operator<(const Number& left, const Number& right) {
    return left.value_ < right.value_;
}
int main() {
    std::set<Number> numbers{};   // okay
    numbers.insert( Number{3} );  // operator< is needed here
}