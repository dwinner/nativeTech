
// https://godbolt.org/z/cv1qfcEd4
#include <iostream>      // cin, cout for input and output
#include <string>        // You need this header of the standard library

void input(
    std::string &name,   // as parameter 
    unsigned &birthYear) 
{ 
    /* Input still without good error handling... */ 
    std::cout << "Name: ";
    std::getline(std::cin, name);  // getline reads a string 
    if(name.length() == 0) {       // length is a method of string 
        std::cout << "You have entered an empty name.\n"; 
        exit(1); 
    } 
    std::cout << "Birth year: "; 
    std::cin >> birthYear; 
} 
int main() { 
    /* data */
    std::string name;              // defines and initializes a string variable 
    unsigned birthJahr = 0; 
    /* input */ 
    input(name, birthJahr); 
    /* calculations */ 
    // ... 
}