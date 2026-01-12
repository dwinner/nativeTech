
// https://godbolt.org/z/KPaPoh4Gn
#include <mutex> // recursive_mutex
#include <iostream>
struct MulDiv {
    std::recursive_mutex mx_;
    int value_;
    explicit MulDiv(int value) : value_{value} {}
    void mul(int x) {
        std::lock_guard lk1(mx_);  // inner
        value_ *= x;
    }
    void div(int x) {
        std::lock_guard lk2(mx_);  // inner
        value_ /= x;
    }
    void muldiv(int x, int y){
        std::lock_guard lk3(mx_);  // outer
        mul(x);
        div(y);
    }
};
int main() {
   MulDiv m{42}; // 3*7*2 *5
   m.muldiv(5, 15);
   std::cout << m.value_ << '\n';  // Output: 14
}