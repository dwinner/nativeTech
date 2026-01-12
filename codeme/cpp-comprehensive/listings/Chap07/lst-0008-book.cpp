
// https://godbolt.org/z/TdrebYodx 
#include <iostream>                     // cout 
#include <cmath>                        // sin 
#include <string> 
#include <vector> 
using std::sin; 
int main() { 
    std::cout << "sin(0.0): " << sin(0.0) << "\n";       // call sin() with literal 
    double angle = 3.1415/2; 
    std::cout << "sin("<<angle<<"): "<<sin(angle)<<"\n"; // call with variable 
    std::string name = "Han Solo"; 
    std::cout << name.length() << "\n"; // call a method 
                                        // ... conceptually like length(name) 
    std::vector<int> data{}; 
    data.push_back(5);                  // further method call with parameter 
    data.push_back(10); 
    std::cout << data.back() << " "; 
    data.pop_back(); 
    std::cout << data.back() << "\n"; 
    data.pop_back(); 
}