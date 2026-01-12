# Listings of Chap19.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 19.1: Call-by-reference as a basis for polymorphism.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/71fE4zKhj
class Car { };
  class VwBus : public Car { };
  class ModelT : public Car { };
void letDrive(Car vehicle) { }     // (ERR) Value parameter copies only base class
void letBrake(Car &vehicle) { }    // Reference parameter
void letHonk(Car *vehicle) { }     // Passed as pointer
int main() {
    VwBus bus{  };                 // automatic variable
    Car *modelT = new ModelT{  };  // dynamically managed
    letDrive(bus);                 // (ERR) gets copied to Car
    letDrive(*modelT);             // (ERR) gets copied to Car
    letBrake(bus);                 // remains VwBus
    letBrake(*modelT);             // remains ModelT
    letHonk(&bus);                 // remains VwBus
    letHonk(modelT);               // remains ModelT
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/71fE4zKhj](https://godbolt.org/z/71fE4zKhj):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/71fE4zKhj
class Car { };
  class VwBus : public Car { };
  class ModelT : public Car { };
void letDrive(Car vehicle) { }     // (ERR) Value parameter copies only base class
void letBrake(Car &vehicle) { }    // Reference parameter
void letHonk(Car *vehicle) { }     // Passed as pointer
int main() {
    VwBus bus{  };                 // automatic variable
    Car *modelT = new ModelT{  };  // dynamically managed
    letDrive(bus);                 // (ERR) gets copied to Car
    letDrive(*modelT);             // (ERR) gets copied to Car
    letBrake(bus);                 // remains VwBus
    letBrake(*modelT);             // remains ModelT
    letHonk(&bus);                 // remains VwBus
    letHonk(modelT);               // remains ModelT
}
```

----------------


## Listing 19.2: The Book class serves the reader and librarian actors.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/fdh4n5x84 
struct Book {
   auto getTitle() { return Title{"The C++ Handbook"s}; }
   auto getAuthor() { return Author{"Torsten T. Will"s}; }
   auto turnPage() { return /* Reference to the next page */ 42; }
   auto getPage() { return "current page content"; }
   auto getLocation() { return /* Shelf number/Book number */ 73; }
};
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/fdh4n5x84](https://godbolt.org/z/fdh4n5x84):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fdh4n5x84 
struct Book {
   auto getTitle() { return Title{"The C++ Handbook"s}; }
   auto getAuthor() { return Author{"Torsten T. Will"s}; }
   auto turnPage() { return /* Reference to the next page */ 42; }
   auto getPage() { return "current page content"; }
   auto getLocation() { return /* Shelf number/Book number */ 73; }
};
```

----------------


## Listing 19.3: The changes of an actor usually only take place in one class.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/Tq9xbGxs4 
struct Book {
   auto getTitle() { return Title{"The C++ Handbook"s}; }
   auto getAuthor() { return Author{"Torsten T. Will"s}; }
   auto turnPage() { return /* Reference to the next page */ 42; }
   auto getPage() { return "current page content"; }
   auto getLocation() { return /* Shelf number/Book number */ 73; }
};
struct BookFinder {
    Catalog catalog;
    auto find(Book& book) { /* Shelf number/Book number */
        catalog.findBookBy(book.getTitle(), book.getAuthor());
    }
};
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/Tq9xbGxs4](https://godbolt.org/z/Tq9xbGxs4):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Tq9xbGxs4 
struct Book {
   auto getTitle() { return Title{"The C++ Handbook"s}; }
   auto getAuthor() { return Author{"Torsten T. Will"s}; }
   auto turnPage() { return /* Reference to the next page */ 42; }
   auto getPage() { return "current page content"; }
   auto getLocation() { return /* Shelf number/Book number */ 73; }
};
struct BookFinder {
    Catalog catalog;
    auto find(Book& book) { /* Shelf number/Book number */
        catalog.findBookBy(book.getTitle(), book.getAuthor());
    }
};
```

----------------


## Listing 19.4: Does not follow the open/closed principle.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/vxTsveGjT](https://godbolt.org/z/vxTsveGjT):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
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
```

----------------


## Listing 19.5: Follows the OCP.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/M6hsYa7or](https://godbolt.org/z/M6hsYa7or):
```cpp
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
```

----------------


## GodboltId:rc67xWWfT

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/rc67xWWfT](https://godbolt.org/z/rc67xWWfT):
```cpp
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
```

----------------


## Listing 19.6: Covariance for return types.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/Gxh4edc8n 
using std::vector;

struct B {};
struct D : public B {};

struct Base1 {
    virtual B& func();
};
struct Derived1 : public Base1 {
    virtual D& func() override;          // D& is covariant
};
struct Base2 {
    virtual B& func();
};
struct Derived2 : public Base2 {
    virtual D func() override;           // (ERR) D is not covariant
};
struct Base3 {
    virtual vector<B> func();
};
struct Derived3 : public Base3 {
    virtual vector<D>& func() override; // (ERR) vector<D>& is not covariant
};
struct Base4 {
    virtual vector<B*>& func();
};
struct Derived4 : public Base4 {
    virtual vector<D*>& func() override; // (ERR) different type, not covariant
};
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/Gxh4edc8n](https://godbolt.org/z/Gxh4edc8n):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Gxh4edc8n 
using std::vector;

struct B {};
struct D : public B {};

struct Base1 {
    virtual B& func();
};
struct Derived1 : public Base1 {
    virtual D& func() override;          // D& is covariant
};
struct Base2 {
    virtual B& func();
};
struct Derived2 : public Base2 {
    virtual D func() override;           // (ERR) D is not covariant
};
struct Base3 {
    virtual vector<B> func();
};
struct Derived3 : public Base3 {
    virtual vector<D>& func() override; // (ERR) vector<D>& is not covariant
};
struct Base4 {
    virtual vector<B*>& func();
};
struct Derived4 : public Base4 {
    virtual vector<D*>& func() override; // (ERR) different type, not covariant
};
```

----------------

