
// https://godbolt.org/z/Y33Tzcf5T 
#include <iostream>                   // cout
#include <vector>                     // container vector
#include <string>                     // stoi
int inputUpTo(int argc, const char* argv[]) {
    /* Determine number */
    int upTo = 0;                     // introduce new variable
    if(argc<=1) {                     // if statement with then and else block
        std::cout << "Up to what number do you want to calculate prime numbers? ";
        if(!(std::cin >> upTo)) {     // check return value
            return -1;                // error in user input
        }
    } else {                          // else part of the if statement
        upTo = std::stoi(argv[1]);
    }
    return upTo;                      // return input
}
std::vector primes{2};                // new vector<int> with initialization
bool isPrime(int n) {
    /* primes must be sorted in ascending order */
    for(int factor : primes) {        // range-based for loop
        if(factor*factor > n)         // too large to be a divisor at all?
            return true;              // … then terminate inner loop early
        if(n%factor==0)               // is divisor?
            return false;             // … then exit
    }
    return true;                      // no divisor found
}
void calculatePrimesUpTo(int upTo) {
    /* Primes calculation */
    /* vector must contain {2} at this point */
    for(int n=3; n<upTo; n=n+2) {     // standard for loop
        if(isPrime(n)) {
            primes.push_back(n);      // is prime – mark as divisor and result
        }
    }
}
void outputPrimes() {
    for(int prime : primes) {         // range based, over all elements
        std::cout << prime << " ";
    }
    std::cout << " ";
}
int main(int argc, const char* argv[]) {
    int upTo = inputUpTo(argc, argv); // declares variable
    if(upTo < 2) { return 1; }        // exit main with nonokay value.
    calculatePrimesUpTo(upTo);
    outputPrimes();
    return 0;
}