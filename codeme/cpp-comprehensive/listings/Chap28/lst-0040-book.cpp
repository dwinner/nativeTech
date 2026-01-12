
// https://godbolt.org/z/8c1r5xrs5 
#include <chrono>
#include <iostream>
using std::chrono::operator""s;  // only make literal suffix available
constexpr auto limit = 10s;
void action(std::chrono::seconds dur) {
    if(dur <= limit) {           // compare
       std::cout << dur.count() << "s\n";
    } else {
       std::cout << "too long!\n";
    }
}
int main() {
    action(4s);   // Output: 4s
    action(20s);  // Output: too long!
}