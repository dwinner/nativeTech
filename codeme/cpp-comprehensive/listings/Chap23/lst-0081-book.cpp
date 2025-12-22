
// https://godbolt.org/z/jbafeEdoW 
#include <iostream>
#include <concepts>    // copyable, C++20
template<std::copyable T, std::copyable U=T>
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t,const U& u) : data01_{t}, data02_{u} {}
    void print(std::ostream& os) const {
        os << data01_ << " : " << data02_ << std::endl;
    }
};
int main() {
    MyPair<double> numbers{11.11, 22.22};
    numbers.print(std::cout);
}