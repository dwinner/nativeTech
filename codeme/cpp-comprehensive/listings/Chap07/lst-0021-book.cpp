
// https://godbolt.org/z/9YcPraT15 
#include <iostream>
void print(int value) { std::cout << "int value: " << value << " "; }
void print(double value) { std::cout << "double value: " << value << " "; }
void print(int v1, double v2) { std::cout << "Values: "<<v1<<", "<<v2<<" "; }
int add(int n, int m) { return n + m; }
double add(double a, double b) { return a + b; }
int main() {
    print( add(3, 4) );         // add(int, int) and print(int)
    print( add(3.25f, 1.5f) );  // add(double, double) and print(double)
    print( 7, 3.25 );           // print(int, double)
}