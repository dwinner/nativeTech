
// https://godbolt.org/z/hhhavG5he 
#include <vector> 
#include <iostream>                    // cout, endl 
int main() {
    std::vector squares{1,4,9,16,25};  // initialize filled
    for(int number : squares)          // number is one square at a time 
        std::cout << number << " "; 
    std::cout << std::endl; 
}