//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hbsbMsGr1
class Year {
    int value_;                         // actually internally used type
public:
    explicit Year(int v) : value_{v} {} // type becomes part of the interface
    int value() { return value_; }      // also in the return
};
int main() {
   Year year{ 2024 };                   // type int
   int val = year.value();              // matching type
}