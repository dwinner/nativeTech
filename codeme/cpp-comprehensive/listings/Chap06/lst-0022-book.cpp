
// https://godbolt.org/z/4G67rso8T 
#include <vector> 
#include <algorithm>                           // count_if 
#include <numeric>                             // iota 
#include <iostream> 
bool even(int n) { return n%2==0; }            // test for even 
int main() { 
    std::vector<int> data(100);                // 100 x zero
    std::iota(data.begin(), data.end(), 0);    // 0, 1, 2, ... 99 
    // counts even numbers 
    std::cout << std::count_if(data.begin(), data.end(), even); 
}