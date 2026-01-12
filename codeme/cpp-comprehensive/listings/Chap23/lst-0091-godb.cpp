//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/eenKej9sj
#include <typeinfo>               // operator typeid
#include <typeindex>              // type_index
#include <map>
#include <string>
#include <iostream>
template <typename T>
void output(std::ostream& os, T val) { // output a type name
    // static: initialized the first time
    static const std::map<std::type_index,std::string> type_names {
        { std::type_index(typeid(const char*)), "const char*"},
        { std::type_index(typeid(char)), "char"},
        { std::type_index(typeid(int)), "int"},
        { std::type_index(typeid(double)), "double"},
        { std::type_index(typeid(bool)), "bool"},
    };
    const std::string name = type_names.at(std::type_index(typeid(T)));
    os << name << ": " << val << '\n';
}
// recursive variadic function template:
template<typename First, typename ... Rest>
void output(std::ostream &os, First first, Rest ... rest) {
    output(os, first);          // single call with the foremost element
    output(os, rest ...);       // recursion with the rest of the elements
}
int main() {
  output(std::cout, 3.1415);                     // normal template
  output(std::cout, "end", 2, 3.14, 'A', false); // recursive variadic function
}