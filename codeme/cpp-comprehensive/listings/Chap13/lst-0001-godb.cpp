//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qo6W3bh5d 
#include <string>
#include <iostream>                   // ostream, cout
namespace plant {
    class Tree {
        std::string name_;
    public:
        explicit Tree(const std::string_view name) : name_{name} {}
        void print(std::ostream& os) const { os << name_; }
    };
    std::ostream& operator<<(std::ostream& os, const Tree& arg)
        { arg.print(os); return os; }
    using ConiferTree = Tree;         // for future extensions …
    using BroadleafTree = Tree;       // … provide for
    namespace exampleNames {          // embedded namespace
        std::string oakName = "Oak";
        std::string beechName = "Beech";
        std::string firName = "Fir";
    } // end namespace exampleNames
} // end namespace plant

int main() {  // main must not be in a namespace
    using namespace plant::exampleNames; // make all example names available
    plant::ConiferTree fir{ firName };
    plant::BroadleafTree oak{ oakName };
    fir.print(std::cout); std::cout << "\n";
    using plant::operator<<;          // without it 'cout << oak' won't work
    std::cout << oak << "\n";
}