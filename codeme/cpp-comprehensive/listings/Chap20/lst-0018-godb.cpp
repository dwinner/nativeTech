//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/W4W61h4Kv
#include <vector>
#include <numeric>     // iota
#include <iostream>
using std::vector; using std::cout;
struct Number {        // representative of a large, expensive object
    unsigned long val_;
    Number(unsigned long val) : val_{val} {}
    Number() : val_{0} {}
};
/* determines whether z is a prime number based on previous prime numbers */
bool isPrime(const Number& z, const vector<Number*> primes) {
    for(Number* p : primes) {
        if((p->val_*p->val_) > z.val_) return true;   // too large
        if(z.val_ % p->val_ == 0) return false;       // is a divisor
    }
    return true;
}
int main() {
    vector<Number> allNumbers(98);  // 98 zero-initialized elements
    std::iota(begin(allNumbers), end(allNumbers), 3); // 3..100
    /* allNumbers now contains {3..100} */
    vector<Number*> primes{};       // stores determined prime numbers
    Number two{2};
    primes.push_back(&two);         // the 2 is needed
    for(Number &z : allNumbers) {   // iterate over all numbers
        if(isPrime(z, primes)) {
            primes.push_back( &z ); // store address
        }
    }
    /* Output the rest */
    for(Number* p : primes)
        cout << p->val_ << " ";
    cout << "\n";
}