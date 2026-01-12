//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/5M8xKabjK
int main() {
    MyValue value1{"ABCD"};
    MyValue value2{"WXYZ"};
    value1 = value2;         // assignment
}