
// https://godbolt.org/z/xnGTM351G
#include <iostream>
using std::cout;
class Mammal {
public:
    void giveBirth() { cout << "Birth!\n"; }
};
class Flying {
public:
    void fly() { cout << "Flight!\n"; }
};
class Bat: public Mammal, public Flying {
public:
    void call() { cout << "Ultrasound!\n"; }
};
int main() {
    Bat bruce{};
    bruce.giveBirth();    // Output: Birth!
    bruce.fly();          // Output: Flight!
    bruce.call();         // Output: Ultrasound!
}