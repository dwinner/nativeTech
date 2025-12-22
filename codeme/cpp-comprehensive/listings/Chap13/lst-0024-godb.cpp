//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/WTWxYa3rd 
class Widget {
    unsigned x = 0, y = 0, w = 0, h = 0; // for example
public:
    unsigned getLeft() const;
    unsigned getTop() const;
    unsigned getRight() const;
    unsigned getBottom() const;
    void setWidth(unsigned w);
    void setHeight(unsigned h);
};