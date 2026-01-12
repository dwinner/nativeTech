
// https://godbolt.org/z/6d5fxn9ed 
auto maxOf2(int a, int b) {
    return a<b ? b : a;   // one return: the compiler determines int
}
auto minOf3(int a, int b, int c) {
    if(a<b) return a<c ? a : c;
    else return b<c ? b : c;
}
auto medianOf3(int a, int b, int c) {
    // more complex, but no problem for the compiler
    return minOf3(maxOf2(a,b), maxOf2(b,c), maxOf2(a,c));
}