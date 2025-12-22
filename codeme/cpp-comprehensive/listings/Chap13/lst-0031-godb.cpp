//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/98x3YsnKc
const Widget createWidget() {  // return as const value
    Widget result{};
    return result;
}
int main() {
    Widget w = createWidget();  // copied into new non-const w
    w.setNumber(100);           // w is non-const, changing is okay
}