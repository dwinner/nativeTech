
#include <iostream> 
#include <string>
using namespace std;            // (ERR) has a global effect; works, but is critical 
void calculate(int n) { 
    /* output divisors */
    cout << "divisors of " << n << " are:\n"; 
    // ...
} 
// ... also in other functions ...