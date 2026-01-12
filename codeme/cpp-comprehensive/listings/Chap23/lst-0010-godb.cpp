//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/aMPj3K7EE 
#include <iostream>
struct Number {
    int value_;
};
template<typename TYPE>
void print(TYPE value) {
    std::cout << value << "\n";
}
int main() {
    print(5);
    print(-10.25);
    print("Flamingo");
    Number seven { 7 };
    print(seven);    // (ERR) cout << seven does not exist
}