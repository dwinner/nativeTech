//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/398GhcbaE 
struct MyNumber {
    int number_;           // variable data field

    explicit MyNumber(int v)
        : number_{v} {}
    MyNumber& operator=(const MyNumber&) = delete; // remove assignment
    MyNumber(const MyNumber&) = delete;            // remove copy
};
int main() {
    MyNumber c1{4};
    MyNumber c2{7};
    c1 = c2;               // (ERR) Error – assignment removed by programmer
    MyNumber c3{c1};       // (ERR) Error – copy removed by programmer
}