
// https://godbolt.org/z/xvfvG8hrd
#include <iostream>                       // cout
using std::cout;
class Increment {
   int amount_;
public:
    explicit Increment(int amount) : amount_{amount} {}
    int operator()(int value) const  {    // makes instances callable
        return value + amount_;
    }
    void clear() {
        amount_ = 0;
    }
};
int main() {
    Increment plusFour{4};                // create instance
    Increment plusEleven{11};             // another instance
    cout << plusFour(8) << "\n";          // Output: 12
    int result = 2 * plusEleven(5) - 7;   // result is 25
    cout << plusEleven(result/5) << "\n"; // Output: 16
    cout << 3 * Increment{1}(7) << "\n";  // Output: 24
    Increment plusZero = plusEleven;
    plusZero.clear();                     // change state
    cout << plusZero(1) << "\n";          // Output: 1
}