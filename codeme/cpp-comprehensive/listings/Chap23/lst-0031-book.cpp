
// https://godbolt.org/z/q719j41er
#include <functional> // function
#include <iostream>   // cout
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
int plus(int a, int b) { return a+b; }
int times(int a, int b) { return a*b; }
int main() {
    std::cout << calculate(3, 4, plus) << "\n";   // Output: 7
    std::cout << calculate(3, 4, times) << "\n";  // Output: 12
}