//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/94McsP7PG 
#include <random>
#include <iostream>
int main() {
    std::default_random_engine e{};
    std::uniform_real_distribution<double> unif{3,7}; // in the half-open interval [3,7)
    double u = unif(e);                         // Generate random number
    std::cout << u << '\n';                     // Example output: 3.52615
}