//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dMTnbT1v4 
#include <iostream>
#include <concepts>    // copyable, C+20
template<std::copyable T, std::copyable U>
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
    std::string month{"January"};
    int temp = -5;
    MyPair<std::string, int> temperature{month, temp};
    temperature.print(std::cout);
}