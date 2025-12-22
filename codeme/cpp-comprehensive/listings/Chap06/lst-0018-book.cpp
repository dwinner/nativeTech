
// https://godbolt.org/z/T9jsEoMj4 
#include <vector>                       // You need this header 
int main() {
    std::vector<int> squares{};         // initialize empty 
    for(int idx = 0; idx<100; ++idx) {
        squares.push_back(idx*idx);      // append an element 
    }
}