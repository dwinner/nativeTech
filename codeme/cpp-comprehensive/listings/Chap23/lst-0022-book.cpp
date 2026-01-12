
// https://godbolt.org/z/bfofn76Ke 
#include <iostream>
class Printer {
    std::ostream& trg_;
public:
    explicit Printer(std::ostream& target)
        : trg_(target)
        {}
    template<typename TYPE>
    Printer& print(const TYP& arg) {
        trg_ << arg;
        return *this;
    }
};
int main() {
    Printer normal(std::cout);
    normal.print(7).print(" ").print(3.1415).print("\n");
    Printer error(std::cerr);
    error.print(8).print(" ").print(2.7183).print("\n");
}