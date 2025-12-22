//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eaGre4bqr 
auto min2(const std::integral auto &a, const std::integral auto &b) {
    return a<b ? a : b;
};

auto min3 = []<std::integral T>(const T &a, const T &b, const T &c) {
    return min2(a, min2(b,c));
};

int main() {
    cout << min3( 3, 7, 2 ) << '\n';
    cout << min3( 8.11, 113.2, -3.1 ) << '\n'; // (ERR) Error: not an integral type
}