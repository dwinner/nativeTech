
// https://godbolt.org/z/h5qond6db 
#include <iostream>                     // cout
class Keyboard {
    Keyboard(const Keyboard&) = delete; // no copy
    const size_t nr_;                   // current number
public:
    static inline size_t count_ = 0;    // counts created instances
    explicit Keyboard() : nr_{count_++} {
        std::cout << "  Keyboard().nr:"<<nr_<<"\n";
    }
};
Keyboard& getKeyboard() {
    std::cout << "  getKeyboard()\n";
    static Keyboard keyboard{};         // static local variable
    return keyboard;
}
void func() {
    std::cout << "kbFunc...\n";
    Keyboard& kbFunc = getKeyboard();
}
int main() {
    std::cout << "kbA...\n";
    Keyboard& kbA = getKeyboard();
    func();
    std::cout << "kbB...\n";
    Keyboard& kbB = getKeyboard();
    std::cout << "count:" << Keyboard::count_ << "\n";
}