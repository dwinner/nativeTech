
// https://godbolt.org/z/vd85f7rMj 
#include <iostream> 
int main() { 
    int arr[] = { 8,3,7,3,11,999,5,6,7 }; 
    int len = 9; 
    for(int i=0, *p=arr; i<len && *p!=999; ++i, ++p) { // first ++i, then ++p 
        std::cout << i << ":" << *p << " "; 
    } 
    std::cout << "\n"; 
    // output: 0:8 1:3 2:7 3:3 4:11 
}