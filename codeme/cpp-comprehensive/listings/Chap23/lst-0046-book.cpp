
// https://godbolt.org/z/YKzsW49G8 
#include <vector>
#include <algorithm>   // sort
// Definitions and further includes as before
int main() {
    vector dwarves{    // initialize as before
    /* sort */
    std::sort(begin(dwarves), end(dwarves));
    // output as before …