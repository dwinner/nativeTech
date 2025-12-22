
#include <iostream>                     // cin, cout, endl
using std::endl;                        // applies globally in this file 
void calculate(int n) {
    using std::cout;                    // applies locally in this function 
    /* output divider */
    cout << "divisors of " << n << " are:\n"; 
    for(int divider=1; divider <= n; ++divider) { 
    if(n % divider == 0)
        cout << divider << ", "; 
    }
    cout << endl; 
}