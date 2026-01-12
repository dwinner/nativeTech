
// https://godbolt.org/z/WYqYe7PoY
#include <iostream>
using std::cout; using std::ostream;
class Base {
    public: int data = 5;
};
class Middle : private Base {
protected: void print() {
        cout << data;   // 'data' is inherited privately here
    }
};
class Ultimately : public Middle {
    public: void go() {
        // 'data' is not visible
        print();        // 'print' is protected visible
    }
};
int main() {
    Ultimately u {};
    u.go();             // output: 5
}