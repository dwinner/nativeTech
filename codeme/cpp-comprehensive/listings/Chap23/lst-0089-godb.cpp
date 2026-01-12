//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/9rzsh5366
#include <string>
#include <tuple>
#include <concepts>             // copyable, C++20
template <std::copyable T, std::copyable U=T> // general case
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t, const U& u) : data01_{t}, data02_{u} {}
};
template <std::copyable T>      // partial specialization, T remains formal
class MyPair<T, std::string> {  // U is specialized to string
    std::tuple<T,std::string> data_;
public:
    MyPair(const T& t, const std::string& str) : data_{t, str} { }
};
template <std::copyable U>      // partial specialization, U remains formal
class MyPair<std::string, U> {  // T is specialized to string
    std::tuple<std::string, U> data_;
public:
    MyPair(const std::string& str, const U& u) : data_{str, u} { }
};
int main() {
  // uses partial specialization
  MyPair<int,std::string> intString{1, "a"};
  MyPair<std::string,int> stringInt{"b", 2};

  MyPair<int,int> intInt{3,4};                     // uses general case
  MyPair intInt2{3,4};                             // also MyPair<int,int>
  MyPair<std::string,std::string> strStr{"c","d"}; // (ERR) ambiguous
}