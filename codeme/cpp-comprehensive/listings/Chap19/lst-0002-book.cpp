
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