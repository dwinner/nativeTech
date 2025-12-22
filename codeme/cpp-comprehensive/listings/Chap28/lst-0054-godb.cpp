//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/ThM8q1zEP
#include <iostream>
#include <ratio>
using std::cout; using std::endl;
int main() {
  using oneThird = std::ratio<1,3>;
  using twoFourths = std::ratio<2,4>;
  cout << oneThird::num << "/" << oneThird::den << endl;     // Output: 1/3
  cout << twoFourths::num << "/" << twoFourths::den << endl; // Output: 1/2
  using sum = std::ratio_add<oneThird,twoFourth>;            // add
  cout << sum::num << "/" << sum::den;                       // Output: 5/6
}