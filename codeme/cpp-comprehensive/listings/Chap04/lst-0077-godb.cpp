//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Tvx4qcrE6 
#include <iostream> 
#include <cmath>   // fpclassify 
#include <limits>  // numeric_limits 
#include <string> 
std::string fpclass(double x) { 
    switch(std::fpclassify(x)) { 
        case FP_INFINITE:  return "infinite"; 
        case FP_NAN:       return "NaN"; 
        case FP_NORMAL:    return "normal"; 
        case FP_SUBNORMAL: return "subnormal"; 
        case FP_ZERO:      return "zero"; 
        default:           return "unknown"; 
    } 
} 
int main() { 
    const auto dmin = std::numeric_limits<double>::min(); 
    std::cout 
      <<"1.0/0.0 is "<<fpclass(1/0.0)<<'\n'   // output: 1.0/0.0 is infinite
      <<"0.0/0.0 is "<<fpclass(0.0/0.0)<<'\n' // Output: 0.0/0.0 is NaN 
      <<"dmin/2 is "<<fpclass(dmin/2)<<'\n'   // Output: dmin/2 is subnormal 
      <<"-0.0 is "<<fpclass(-0.0)<<'\n'       // Output: -0.0 is zero 
      <<"1.0 is "<<fpclass(1.0)<<'\n';        // Output: 1.0 is normal 
}