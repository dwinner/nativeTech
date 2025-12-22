
vector<Number*> primePointers(unsigned long upTo) { // (ERR) Vector of pointers is suspicious
    vector<Number> allNumbers;
    vector<Number*> primes{};
    // …
    for(Number &z : allNumbers)
        if(isPrime(z, primes))
            primes.push_back( &z ); // store address
    return primes;   // (ERR) Pointers to function-local objects
}