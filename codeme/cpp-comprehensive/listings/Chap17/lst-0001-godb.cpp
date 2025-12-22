//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/K1714eWr9 
struct Type {
    char* data_;             // raw pointer can cause unclear ownership
    explicit Type(int n) : data_(new char[n]) {}
    ~Type() { delete[] data_; }             // you need the destructor

    Type(const Type&) = delete;             // do not allow copying
    Type& operator=(const Type&) = delete;  // no assignment please
    Type(Type&&) = delete;                  // no moving
    Type& operator=(Type&&) = delete;       // no move assignment operator
};