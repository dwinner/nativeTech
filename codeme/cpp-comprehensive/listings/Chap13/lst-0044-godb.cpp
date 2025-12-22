//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zMbnYzo17 
template<typename T>
auto deref(T t) {
    if constexpr (std::is_pointer_v<T>) {
        return *t;
    } else {
        return t;
    }
}
int main() {
    int i = 42;
    std::cout << deref(i) << '\n';         // directly the value
    auto p = std::make_unique<int>(73);
    std::cout << deref(p.get()) << '\n';   // dereferenced pointer
}