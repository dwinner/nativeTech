
// https://godbolt.org/z/e6xT638ch 
#include <functional> // subtract, minus, bind
#include <iostream>
using std::cout;
int subtract(int a, int b) { return a - b; }
int main() {
    using namespace std::placeholders;
    cout << subtract(9, 3) << '\n'  ;  // Output: 6
    auto minus3 = std::bind(subtract, _1, 3);
    cout << minus3(9) << '\n';         // Output: 6
    auto from9 = std::bind(subtract, 9, _1);
    cout << from9(3) << '\n';          // Output: 6
    auto againMinus3 = std::bind(std::minus<int>{}, _1, 3);
    cout << againMinus3(9) << '\n';    // Output: 6
}