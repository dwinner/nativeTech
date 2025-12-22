
// https://godbolt.org/z/67EjhMf54
class Car {
public:
  Car() : engine_{8} { }            // initializes a car with 8 cylinders
  void start() { engine_.start(); } // starts the car by starting the engine
private:
  Engine engine_;                   // Car has-a engine
};