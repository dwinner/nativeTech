
// https://godbolt.org/z/YonbGKj8M 
#include <iostream>                       // cout
int main() {
    using std::cout;
    Num a{1};
    *a = 7;                               // operator* also returns int&
    a += Num{3};                          // Increment with Num
    cout << ( ++( ++a ) ) << "\n";        // Output: 12
    a -= 2;                               // Variation with int
    cout << --(--a) << "\n";              // Output: 8
    Num b{99};
    cout << (a<b ? "yes\n" : "xxx\n");    // Output: yes
    cout << (a>b ? "xxx\n" : "no\n");     // Output: no
    b /= Num{3};           // b: 33
    b %= Num{10};          // b: 3
    b <<= 4;               // b: 48
    b >>= 2;               // b: 12
    Num c = b / Num{3} + a * Num{2};      // c: 20
}