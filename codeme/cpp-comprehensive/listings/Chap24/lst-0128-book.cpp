
// https://godbolt.org/z/hf8rqz5x5
#include <iostream>
#include <iomanip> // setw
#include <valarray>
using namespace std;

/* Print matrix */
template<class T>
void printMatrix(ostream&os, const valarray<T>& a, size_t n) {
    for(size_t i = 0; i < (n*n); ++i) {
        os << setw(3) << a[i];        // Print value
        os << ((i+1)%n ? ' ' : '\n'); // next line?
    }
}

/* Matrix cross product */
template<class T>

valarray<T> matmult(
        const valarray<T>& a, size_t arows, size_t acols,
        const valarray<T>& b, size_t brows, size_t bcols)
{
    /* Condition: acols==brows */
    valarray<T> result(arows * bcols);
    for(size_t i = 0; i < arows; ++i) {
      for(size_t j = 0; j < bcols; ++j) {
        auto row = a[slice(acols*i, acols, 1)]; // Row
        auto col = b[slice(j, brows, bcols)];   // Column
        result[i*bcols+j] = (row*col).sum();    // Cross product of row a[i] and 
                                                // column b[j]
      }
    }
    return result;
}

int main() {
    constexpr int n = 3;
    valarray ma{1,0,-1,  2,2,-3,  3,4,0};       // 3 x 3 matrix
    valarray mb{3,4,-1,  1,-3,0,  -1,1,2};      // 3 x 3 matrix
    printMatrix(cout, ma, n);
    cout << "  -times-\n";
    printMatrix(cout, mb, n);
    cout << "  -results in-:\n";
    valarray<int> mc = matmult(ma, n,n, mb, n,n);
    printMatrix(cout, mc, n);
}