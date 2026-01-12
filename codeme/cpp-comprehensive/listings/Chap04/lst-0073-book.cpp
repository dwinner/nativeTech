
// https://godbolt.org/z/zGso3xGcj 
#include <vector> 
#include <cstddef> // size_t 
int main() { 
    std::vector<int> data = { 100, -4, 6'699, 88, 0, } ; 
    int sum = 0; 
    for(size_t idx = 0; idx < data.size(); ++idx) { // a specific int type 
        sum += data[idx]; 
    }
 }