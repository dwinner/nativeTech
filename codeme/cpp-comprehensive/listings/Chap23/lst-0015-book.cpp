
// https://godbolt.org/z/c9xzM7EhP 
#include <array>
#include <iostream> // cout
using std::array; using std::cout;
template<size_t SIZE>
array<int,SIZE> createArray() {
    array<int,SIZE> result{};
    return result;
}
int main() {
    auto data = createArray<5>();
    data[3] = 33;
    for(auto e : data) cout << e << " ";
    cout << "\n";
}