
struct MyNumber {

    const int number_;         // constant data field

    explicit MyNumber(int v)
        : number_{v}           // initialization of the constant data field
        {}
};

int main() {
    MyNumber c1{4};
    MyNumber c2{7};
    c1 = c2;                   // (ERR) Error: Assignment removed by compiler
}