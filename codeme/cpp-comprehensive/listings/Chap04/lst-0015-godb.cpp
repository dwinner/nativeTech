//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/E4W9KhbnK
void calculate(int n) {                            // a separate function 
    using namespace std;                           // for std::cout and std::endl 
    /* output divisors */ 
    cout << " divisors of " << n << " are:\n"; 
    for(int divider=1; divider <= n; ++divider) {  // instead of divider=divider+1 
        if(n % divider == 0) 
            cout << divider << ", "; 
    } 
    cout << endl; 
}