
// https://godbolt.org/z/1zq4jfdYb
#include <string>
#include <iostream>
using std::string; using std::ostream;
class Shape {
    string color_;
public:
    virtual double calcArea() const = 0; // pure virtual method
    string getColor() const { return color_; }
    void setColor(const string& color) { color_ = color; }
    virtual ~Shape() {}
};
class Square : public Shape {
    double len_;
public:
    explicit Square(double len) : len_{len} {}
    double calcArea() const override { return len_*len_; }
};
class Circle : public Shape {
    double rad_;
public:
    explicit Circle(double rad) : rad_{rad} {}
    double calcArea() const override { return 3.1415*rad_*rad_; }
};
struct Calculator {
    Shape& shape_;
    Calculator(Shape& shape) : shape_{shape} { }
    void run(ostream& os) const {
        os << "The area of the shape is " << shape_.calcArea() << "\n";
    }
};
int main() {
    Square square {5.0};
    Calculator ti { square };
    ti.run(std::cout); // Output: The area of the shape is 25
}