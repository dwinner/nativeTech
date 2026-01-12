//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nMcsbxvvv 
#include <iostream> 
void printBin(int x) { 
  while(x>0) {       // done? 
    int a = x/2;   // division by 2 
    int b = x%2;   // modulo, remainder of the division 
    std::cout << x <<" / 2 = " << a << ", remainder " << b<<'\n'; // output 
    x = a; 
  } 
} 
int main() { 
  printBin(412); 
}