
// https://godbolt.org/z/816Ys17aT 
#include <iostream>
using std::cout;
template<typename TYPE>
  void func(TYP a) { cout << a <<" TYP\n"; }
void func(int a) { cout << a << " int\n"; }
int main() {
    func<int>(8); // Output: 8 TYP
    func(8);      // Output: 8 int
}