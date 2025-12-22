//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/veEq4dcET 
struct Day {
    Day(int a, int b) : Day{} {       // delegates
        if(a==0 || b == 0)
            throw 666;                // throws exception
    }
    Day() {}
};
int main() {
    try {
        Day day{1,2};
    } catch(int) { }
}