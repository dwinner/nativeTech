//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zqKvMedz7 (full example)
class Date;                         // forward declaration
class Year : public Value {
public:
    explicit Year(int v) : Value{v, 4} {}
    Date easter() const;            // declare new method
};
// Declare Month, Day, and Date here. Then:
Date Year::easter() const {         // define new method
    const int y = value_;
    int a = value_/100*1483 - value_/400*2225 + 2613;
    int b = (value_%19*3510 + a/25*319)/330%29;
    b = 148 - b - (value_*5/4 + a - b)%7;
    return Date{Year{value_}, Month{b/31}, Day{b%31 + 1}};
}
int main() {
    using std::cout;
    Year year{2025};
    cout << year.easter() << "\n";  // Output: 2025-04-20
}