
// https://godbolt.org/z/dMvqG85fr 
double add(double a, double b) { return a + b; }
double add(int, int) = delete;   // prohibit add(3,4)