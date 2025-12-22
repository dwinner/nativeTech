//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/863b1z4GY 
int main() { 
    std::cout << std::setprecision(15) << std::fixed; 
    triangle(100'000.0f, 99'999.999'79f, 0.000'29f); 
    triangle(100'000.0,  99'999.999'79,  0.000'29); 
}