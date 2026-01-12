//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fnsWhn5T3 
// #includes, inputUpTo(), outputPrimes() and main() as before 
std::vector<int> primes{2};
void calculatePrimesUpTo(int upTo) {
    /* Primes calculation */
    /* vector must contain {2} at this point */
    for(int n=3; n<upTo; n=n+2) {
        for(int factor: primes) {
            if(factor*factor > n)
                goto prime;     // using goto, because a break ...
            if(n%factor==0)
                goto notPrime;  // ... doesn't work over two loops.
        }
      prime:  ;                 // Target of the jump before push_back
        primes.push_back(n);    // n is prime! Remember as divisor and result
      notPrime:  ;              // Target of the jump after push_back
    }
}