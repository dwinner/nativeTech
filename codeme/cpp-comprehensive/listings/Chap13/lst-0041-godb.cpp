//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yv85s9T6v 
constexpr size_t doubleIfTooSmall1(size_t value) {
   return value < 100 ? value*2 : value; // returns double if less than 100
}
std::array<int, doubleIfTooSmall1(50)> arr {};