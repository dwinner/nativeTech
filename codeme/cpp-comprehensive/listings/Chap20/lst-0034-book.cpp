
// https://godbolt.org/z/h3q839x8r
#include <memory>      // unique_ptr
#include <iostream>    // cout
std::unique_ptr<int[]> createData(size_t sz) {
    return std::unique_ptr<int[]>(new int[sz]);
}
void fibonacci(int data[], int* end) {
    for(int* p = data+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);
    }
}
std::ostream& print(std::ostream &os, int data[], int* end) {
    for(int const* p = data; p != end; ++p)
        std::cout << *p << " ";
    return os;
}
int main() {
    std::unique_ptr<int[]> data { createData(10) };
    data[0] = 1; // set values in the array through the unique_ptr
    data[1] = 1;
    fibonacci(data.get(), data.get()+10); // get the C-array pointer with get()
    print(std::cout, data.get(), data.get()+10) << "\n";
}