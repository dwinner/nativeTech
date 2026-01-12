
// https://godbolt.org/z/vxTsveGjT 
#include <vector>
#include <memory>         // unique_ptr
#include <iostream>       // cout
enum class ShapeTag { CIRC, RECT };
struct Shape {            // Data
    ShapeTag tag_;
    double v1_, v2_;
    Shape(double w, double h) : tag_{ShapeTag::RECT}, v1_{w}, v2_{h} {}
    Shape(double r) : tag_{ShapeTag::CIRC}, v1_{r}, v2_{0} {}
};
class AreaCalculator {    // Logic
public:
    double area(const std::vector<std::unique_ptr<Shape>> &shapes) const {
        double result = 0;
        for(auto &shape :  shapes) {
            switch(shape->tag_) {
            case ShapeTag::CIRC:
                 result += 3.1415 * shape->v1_ * shape->v1_;
                 break;
            case ShapeTag::RECT:
                 result += shape->v1_*shape->v2_;
                 break;
            }
        }
        return result;
    }
};
int main() {
    std::vector<std::unique_ptr<Shape>> data{};
    data.push_back(std::make_unique<Shape>(10.));     // a circle
    data.push_back(std::make_unique<Shape>(4., 6.));  // a rectangle
    // calculate
    AreaCalculator calc{};
    std::cout  << calc.area( data ) << "\n";
}