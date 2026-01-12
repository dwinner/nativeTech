
// https://godbolt.org/z/59WKfxenE 
class Base {
public:
    Base() {}                 // null-argument constructor
    explicit Base(int i) {}   // one argument
    Base(int i, int j) {}     // two arguments
    void func() {};           // method
};

class Derived : public Base { // no own constructor
};

int main() {
    Base b0{};                // okay, null-argument constructor
    Base b1{12};              // okay, one argument
    Base b2{6,18};            // okay, two arguments
    Derived d0{};             // okay, compiler generates default constructor
    d0.func();                // okay, method is inherited
    Derived d1{7};            // (ERR) Error: no constructor for one argument
    Derived d2{3,13};         // (ERR) Error: no constructor for two arguments
}