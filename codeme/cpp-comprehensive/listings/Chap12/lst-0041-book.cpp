
// https://godbolt.org/z/PaG9nTs7s 
class Rect {
    int area_;          // private data

public:
    int x_, y_;
    void set(int x, int y) { x_=x; y_=y; area_=x_*y_; }
    int calc() { return area_; }
    Rect() = default;   // let the compiler generate a constructor
};
class Pow {
    int result_;        // private data; holds 'base' raised to 'exp'
public:
    int base_, exp_;
    void set(int b, int e) { /* ... */ }
    int calc() { return result_; }
    Pow() : result_{1},base_{1},exp_{1} {} // initialize sensibly
};