
#include <iostream>                            // for std::cin, std::cout, std::endl 
#include <string>                              // for std::stoi 
void calculate(int n) {
    using namespace std;                       // for std::cout and std::endl 
    /* output divisors */
    cout << "divisors of " << n << " are:\n";  // cout instead of std::cout 
    for(int divider=1; divider <= n; ++divider) { 
    if(n % divider == 0)
        cout << divider << ", ";               // cout instead of std::cout 
    }
    cout << endl; 
}