
// https://godbolt.org/z/MfKzd8bxa
#include <iostream>                           // cout
using std::cout;

class Add {
   int amount_;
public:
    explicit Add(int amount) : amount_{amount} {}
    int add(int value) const  {               // instead of operator()
        return value + amount_;
    }
    void clear() {
        amount_ = 0;
    }
};
int main() {
    Add plusFour{4};                          // create instance
    Add plusEleven{11};                       // another instance
    cout << plusFour.add(8) << "\n";          // Output: 12
    int result = 2 * plusEleven.add(5) - 7;   // result is 25
    cout << plusEleven.add(result/5) << "\n"; // Output: 16
    cout << 3 * Add{1}.add(7) << "\n";        // Output: 24
    Add plusZilch = plusEleven;
    plusZilch.clear();                        // change state
    cout << plusZilch.add(1) << "\n";         // Output: 1
}