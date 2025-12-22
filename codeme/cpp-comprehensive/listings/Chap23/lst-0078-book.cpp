
// https://godbolt.org/z/YaEW5aTra
#include <string>
#include <vector>
#include <set>
#include <tuple>
#include <memory> // shared_ptr
template <typename T>
class MyStuff {
    T data_;
public:
    MyStuff() : data_{} {}             // default constructor
    MyStuff(const T& d) : data_{d} { } // copy constructor
    T getStuff() const { return data_; }
};
class IntValue {
    int val_;
public:
    explicit IntValue(int val=0) : val_(val) {}
    int getInt() const { return val_; }
};
int main() {
    MyStuff intStuff(12);               // becomes MyStuff<int>
    std::vector vs{ 1,2,3,4 };          // becomes vector<int>
    MyStuff ivalStuff{ IntValue{33} };  // becomes MyStuff<IntValue>
    std::tuple tpl{ 23, 'a' };          // becomes tuple<int,char>
    std::set twoThree (vs.begin()+1, vs.end()-1); // becomes set<int>
    // The compiler cannot deduce:
    MyStuff<double> dblStuff;               // no constructor argument
    std::vector<char> vcs(10);              // size 10, but of what type?
    std::shared_ptr<int> ptr{new int{88}};  // no rule for int*
}