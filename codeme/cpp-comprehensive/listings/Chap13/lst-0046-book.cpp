
// https://godbolt.org/z/44KMf1fWr 
template<auto N>
constexpr auto fibonacci()    {
    if constexpr (N>=2) {
        return fibonacci<N-1>() + fibonacci<N-2>();
    } else {
        return N;
    }
}

int main() {
    std::cout << fibonacci<10>() << '\n';  // Output: 55
    std::cout << fibonacci<20>() << '\n';  // Output: 6765
}