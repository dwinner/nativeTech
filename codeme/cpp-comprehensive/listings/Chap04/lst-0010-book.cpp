
// https://godbolt.org/z/ceT6o5zxd
#include <iostream>    // for std::cin, std::cout, std::endl 
#include <string>      // std::stoi

void calculate(int n) {                           // a separate function
    using namespace std;                          // for std::cout and std::endl 
    /* output divisors */ 
    cout << "divisors of " << n << " are:\n"; 
    if(n == 0) { cout << "0\n"; return; }         // 0 is divisor of 0 
    for(int divider=1; divider <= n; ++divider) { // for divider=divider+1 
    if(n % divider == 0) 
      cout << divider << ", "; 
    } 
    cout << endl; 
} 
int main(int argc, const char* argv[]) {          // Arguments for main 
    /* Determine number */ 
    int value = 0; 
    if(argc<=1) { 
        std::cout << "Enter a number: "; 
        std::cin >> value;                        // read into variable wert 
        if(!std::cin) {                           // check whether reading worked 
            return 1;                             // error on user input 
        }    
    } else { 
        value = std::stoi(argv[1]); 
    }
    calculate(value);                             // function call 
    return 0; 
}