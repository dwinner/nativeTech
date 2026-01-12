
// https://godbolt.org/z/Ex6Mv6xcz 
unsigned fib(unsigned n) {
    if(n==0) { return 0; }
    if(n==1) { return 1; }
    unsigned a = 0;
    unsigned b = 1;
    unsigned sum = 1;
    while(n>1) {
        sum += a;
        a = b;
        b = sum;
        n -= 1;
    }
    return sum;
}