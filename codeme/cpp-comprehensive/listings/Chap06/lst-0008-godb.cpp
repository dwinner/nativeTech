//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/McqfcWb8j 
#include <fstream> 
int main(int argc, char* argv[]) {
    std::ofstream myOutput{"output1.txt"}; 
    myOutput << "line 1\n"; 
    myOutput << "line 2\n"; 
}