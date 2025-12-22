
// https://godbolt.org/z/7GEWYeoc9
#include <vector>
#include <iostream>
using std::vector; using std::cout; using std::ostream;
ostream& printVector(ostream& os, const vector<int> &arg) { // Helper function
    for(int w : arg) os << w << " "; return os;
}
int main() {
    vector<int> values{ };
    values.reserve(50);                  // Guarantee space for 50 values
    int *largest = nullptr;              // Initialize with a special value
    for(int w : { 20, 2, 30, 15, 81, 104, 70, 2, }) {
        values.push_back(w);
        if(!largest || *largest < w ) {  // Dereference to value
            largest = &(values.back());  // Remember new address; hence not '*'
        }
    }
    printVector(cout, values) << "\n";   // Output: 20 2 30 15 81 104 70 2
    // largest now contains the address of 104:
    *largest = -999;                     // dereference; also overwrite value
    printVector(cout, values) << "\n";   // Output: 20 2 30 15 81 –999 70 2
}