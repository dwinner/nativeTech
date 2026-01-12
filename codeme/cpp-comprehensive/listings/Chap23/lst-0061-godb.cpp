//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nenW8aM81 
auto min2 = []<typename T>(const T &a, const T &b) {
    return a<b ? a : b;
};
auto min3 = []<typename T>(const T &a, const T &b, const T &c) {
    return min2(a, min2(b,c));
};