
// https://godbolt.org/z/jqzTMxYKv 
template <std::copyable T>      // C++20 concept
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; }
    T getData() const { return data_; }
};