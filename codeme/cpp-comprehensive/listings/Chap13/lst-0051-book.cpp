
// https://godbolt.org/z/sPorKnEbd 
#include <iostream>
class Data {
    int value_;
    mutable size_t getCount_{0};
  public:
    explicit Data(int v) : value_{v} { }
    ~Data() {
        std::cout << "get was used " << getCount_ << " times\n";
    }
    int get() const {
        ++getCount_;
        return value_;
    }
};
int main() {
    Data d{42};
    for(int i=0; i<10; ++i) { d.get(); }
} // Output: get was used 10 times