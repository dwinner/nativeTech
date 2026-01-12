
// https://godbolt.org/z/n8jn7v5vb 
#include <mdspan>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    // 1D: 12 elements
    vector v{1,2,3,4,5,6,7,8,9,10,11,12};
    // 2D: as 2 rows with 6 ints each
    auto ms2 = mdspan(v.data(), 2, 6);
    // 3D: as a cuboid with 2 layers, 3 rows, 2 columns
    auto ms3 = mdspan(v.data(), 2, 3, 2);
    // write via 2D view
    for (auto i = 0; i != ms2.extent(0); ++i)
      for (auto j = 0; j != ms2.extent(1); ++j)
        ms2[i, j] = i * 100 + j;  // write via multidimensional index
    // read via 3D view
    for (auto i = 0; i != ms3.extent(0); ++i) {
      cout << "Level " << i << ":\n";
      for (auto j = 0; j != ms3.extent(1); ++j) {
        for (auto k = 0; k != ms3.extent(2); ++k)
          cout << " " << ms3[i, j, k];  // read via multidimensional index
        cout << '\n';
      }
    }
    // Output: Level 0: 0 1, 2 3, 4 5, Level 1: 100 101, 102 103, 104 105
}