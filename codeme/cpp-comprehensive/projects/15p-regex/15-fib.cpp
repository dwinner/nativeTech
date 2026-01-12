int fib(int n) {
    if(n==0) { return 0; }
    if(n==1) { return 1; }
    int a = 0;
    int b = 1;
    int sum = 1;
    while(n>1) {
        sum += a;
        a = b;
        b = sum;
        n -= 1;
    }
    return sum;
}
