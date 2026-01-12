//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rc67xWWfT 
#include <iostream>
struct Point { int x, y; };
class Rectangle {
protected:
    Point origin_;  int width_;  int height_;
public:
    Rectangle(Point o, int w, int h) : origin_{o}, width_{w}, height_{h} {}
    virtual void setHeight(int height) { height_ = height; }
    virtual int  getHeight() const { return height_; }
    virtual void setWidth(int width) { width_ = width; }
    virtual int getWidth() const { return width_; }
    virtual int getArea() const { return width_ * height_; }
};
class Square : public Rectangle {
public:
    Square(Point o, int wh) : Rectangle{o, wh, wh} {}
    void setHeight(int wh) override { width_ = height_ = wh; }
    void setWidth(int wh) override { width_ = height_ = wh; }
};
void areaCheck(Rectangle &rect) {
    rect.setWidth(5);
    rect.setHeight(4);
    auto areaValue = rect.getArea();
    if(areaValue != 20) {
        std::cout << "error!\n";
    } else {
        std::cout << "all fine\n";
    }
}
int main() {
    Rectangle rect{ {0,0}, 0,0 };
    areaCheck( rect );           // Output: all fine
    Square square{ {0,0}, 0 };
    areaCheck( square );         // Output: error!
}