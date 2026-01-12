//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/b9arcEMM1 
constinit auto SZ = 10*1000-1;                  // global variable
size_t autoincrement() {
    static constinit size_t i = 0;              // local static variable
    return i++;
}
class BraitenbergVehicle {
    inline static constinit size_t count_ = 0; // class variable
public:
    size_t id_;
    BraitenbergVehicle() : id_{++count_} {}
    ~BraitenbergVehicle() { --count_; }
};