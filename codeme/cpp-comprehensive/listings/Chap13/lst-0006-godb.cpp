//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/e3d55zvT8 
// modul.hpp
#include <string>
#include <iostream>
namespace plant {
    class Tree {
        std::string name_;
    public:
        explicit Tree(const std::string_view name);
        void print(std::ostream& os) const;
    };
    std::ostream& operator<<(std::ostream& os, const Tree& arg);
}
// modul.cpp
#include "modul.hpp"
namespace {  // anonymous namespace
    std::string PREFIX = "TREE:";
    void printInfo(std::ostream& os) {
        os << "Author: Torsten T. Will\n";
    }
}
bool debug = false;  // global, no namespace
namespace plant {
    Tree::Tree(const std::string_view name)
        : name_{name} {}
    void Tree::print(std::ostream& os) const {
        os << PREFIX << name_;
    }
    std::ostream& operator<<(std::ostream& os, const Tree& arg) {
        if(debug) printInfo(os);
        arg.print(os); return os;
    }
}
// main.cpp
#include "modul.hpp"
int main() {
    plant::Tree x{"x"};
    x.print(std::cout); std::cout << "\n";
}