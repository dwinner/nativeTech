//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/M6hsYa7or
#include <vector>
#include <memory>    // unique_ptr
#include <iostream>  // cout
struct Shape {
    virtual ~Shape() {}
    virtual double area() const = 0; // abstract
};
class Rectangle : public Shape {
    double w_, h_;
public:
    Rectangle(double w, double h) : w_{w}, h_{h} {}
    double area() const override { return w_ * h_; }
};
class Circle : public Shape {
    double r_;
public:
    Circle(double r) : r_{r} {}
    double area() const override { return 3.1415*r_* r_; }
};
class AreaCalculator { // Logic
public:
    double area(const std::vector<std::unique_ptr<Shape>> &shapes) const {
        double result = 0;
        for(auto &shape :  shapes) {
            result += shape->area();
        }
        return result;
    }
};
int main() {
    std::vector<std::unique_ptr<Shape>> data{};
    data.push_back(std::make_unique<Circle>(10.));        // a circle
    data.push_back(std::make_unique<Rectangle>(4., 6.));  // a rectangle
    // calculate
    AreaCalculator calc{};
    std::cout  << calc.area( data ) << "\n";
}