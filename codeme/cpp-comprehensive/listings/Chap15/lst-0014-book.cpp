
// https://godbolt.org/z/h15eGahjh 
class Base {
public:
    Base() {}
    explicit Base(int i) {}
    Base(int i, int j) {}
    void func() {};           // method
};

class Derived : public Base {
public:
    using Base::Base;         // importing all parent class constructors
};

int main() {
    Derived d0{};             // okay, imported, no longer generated
    Derived d1{7};            // okay, was imported
    Derived d2{3,13};         // okay, was imported
}