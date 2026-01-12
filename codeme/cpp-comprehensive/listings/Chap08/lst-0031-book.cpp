
// Excerpt
std::vector<int> primes{2};    // vector is a container, ready for a ranged for
    void isPrime(int n) {
    /* primes must be sorted in ascending order */
    for(int factor : prims) {  // range-based for loop
        if(factor*factor > n)
            return true;
        if(n%factor==0)
            return false;
    }
    return true;
}
// …

void outputPrimes() {
    for(int i=1; int prime : prims) {  // range-based for loop
        std::cout << i++ << ". Prime number: " << prime << " ";
    }
    std::cout << " ";
}
// …