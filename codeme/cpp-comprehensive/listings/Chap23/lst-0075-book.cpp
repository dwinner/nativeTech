
// https://godbolt.org/z/4fK5WdoG8 
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d);
    T getData() const;
};
template <std::copyable T>
void MyContainer<T>::setData(const T& d) {
    data_ = d;
}
template <std::copyable T>
T MyContainer<T>::getData() const {
    return data_;
}