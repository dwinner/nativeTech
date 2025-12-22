
std::vector<int> primes = {2};        // global variable
bool isPrime(int n) { // own function 
    for(int factor : primes) {        // access to global variable 
        if(factor*factor > n)         // access to parameter 
            return true; 
        if(n%factor==0) 
            return false; 
    } 
    return true; 
}
void calculatePrimesUpTo(int limit) { // another custom function 
    for(int n=3; n<limit; n=n+2) { 
        if(isPrime (n)) {             // use custom function 
            primes.push_back(n); 
        }
    }
 }