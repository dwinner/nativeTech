
// ... extract ...
bool isPrime(int n);                  // Declaration of function defined later 
void calculatePrimesUpTo(int limit) { 
    for(int n=3; n<limit; n=n+2) { 
        if(isPrime(n)) {             // Use of function defined later 
            primes.push_back(n); 
        }
    }
}
bool isPrime(int n) {                // Definition after useage 
    // ... as before ... 
}