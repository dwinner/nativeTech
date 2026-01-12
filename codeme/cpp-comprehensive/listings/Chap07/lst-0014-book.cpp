
// https://godbolt.org/z/oh91YvaqT 
#include <iostream> 
void twice(double &num) {      // output parameter as modifiable reference 
    num *= 2.0; 
}

int main() { 
    double num = 7.25; 
    twice(num); 
    std::cout << num << "\n";  // now 14.5 
}