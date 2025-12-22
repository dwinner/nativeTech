//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dMvqG85fr 
double add(double a, double b) { return a + b; }
double add(int, int) = delete;   // prohibit add(3,4)