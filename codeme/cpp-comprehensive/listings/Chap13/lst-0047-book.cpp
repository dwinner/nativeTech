
// https://godbolt.org/z/v3x7nGs3f 
int get_input() {
    return 50;  // or read something from a file or so
}
constexpr auto calculate_1(int input) {
    return input * 2;
}
consteval auto calculate_2(int input) {
    return input * 2;
}
int main() {
    int input = get_input();
    auto a = calculate_1(77);     // at compile time … maybe computable
    auto b = calculate_1(input);  // … computable, but valid
    auto c = calculate_2(77);     // … computable
    auto d = calculate_2(input);  // (ERR) … not computable, invalid
}