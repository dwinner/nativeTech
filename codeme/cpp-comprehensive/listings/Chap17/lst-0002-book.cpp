
// https://godbolt.org/z/nbh9aozPM 
#include <vector>
#include <memory>            // unique_ptr, shared_ptr
struct Type1 {               // automatic complete copy of the resource
    std::vector<char> data_;
    Type1(int n) : data_(n) {}
};
struct Type2 {               // copy prohibited, move allowed
    std::unique_ptr<int[]> data_;
    Type2(int n) : data_(new int[n]) {}
};
struct Type3 {               // copy allowed, resource is then cleanly shared
    std::shared_ptr<Type1> data_;
    Type3(int n) : data_(std::make_shared<Type1>(n)) {}
};