
// https://godbolt.org/z/fev879zaK
#include <iostream>
const int& a_or_b(int choice) {
    static const int a = 42;
    static const int b = 73;
    if(choice==1)
        return a; // return const& to inner variable a
    else
        return b; // return const& to inner variable b
}
template<typename TYPE>
TYPE add(TYPE a, TYPE b) {
    return a + b;
}
int main() {
    auto res = add(
        a_or_b(0),   // const int&
        a_or_b(1) ); // const int&
    std::cout << res << "\n"; // Output: 115
}