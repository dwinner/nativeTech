
// https://godbolt.org/z/ETr73T5a9
class Engine {
public:
  Engine(int numCylinders);
  void start();                   // start engine
};
class Car : private Engine {      // Car has a engine
public:
  Car() : Engine{8} { }           // initializes a car with 8 cylinders
  using Engine::start;            // starts car by starting the engine
};