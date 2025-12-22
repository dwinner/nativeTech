
// https://godbolt.org/z/sTosKfocx 
#include <iostream>
class Mechanic;
class Thing {
    int value_; // private
public:
    explicit Thing(int value) : value_{value} {}
    void increment() { ++value_; }
    std::ostream& print(std::ostream& os) const { return os << value_; }
    friend class Mechanic;
};
class Mechanic {
    const Thing &thing_;
public:
    explicit Mechanic(const Thing &thing) : thing_{thing} {}
    auto getThingValue() const {
        return thing_.value_;        // access to private member of Thing
    }
};
int main() {
    Thing thing{45};
    thing.print(std::cout) << '\n';                // output: 45
    Mechanic mechanic{thing};
    thing.increment();                             // change internal value
    std::cout << mechanic.getThingValue() << '\n'; // output: 46
}