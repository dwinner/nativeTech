//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fM4nsx9GP 
struct Widget {
    int num_ = 0;
    void setNumber(int x) {    // a non-const method
        num_=x;
    }
};
Widget createWidget() {        // Return by value
    Widget result{};            // Create
    return result;
}
int main() {
    Widget w = createWidget(); // Return by value creates a copy
    w.setNumber(100);          // changing is naturally okay, w is non-const
}