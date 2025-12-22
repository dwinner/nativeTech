
// https://godbolt.org/z/Mdj7bGvar 
#include <iostream>                     // Include modules/libraries 
int main()                              // main() is the start of the program 
{ 
    int value = 100;                    // Variable with initial value 
    std::cout << "Divisors of " << value << " are:\n";        // output of text 
    for(int divider=1; divider <= value; divider = divider+1) // loop from 1 to 100 
    {                                   // begin of the repetition part
        if(value % divider == 0)        // test for conditional execution 
        std::cout << divider << ", ";   // only if the test is positive 
    }                                   // end of the loop 
    std::cout << "\n";                  // single output 
    return 0;                           // means end of program in main() 
}                                       // end of main()