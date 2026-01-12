
// https://godbolt.org/z/zb1T7Kr3G 
#include <iostream>
#include <format>
void prints(short s, int i, float f, double d) {
  std::cout << std::format("short: {} int: {} float: {:.2f} double: {:.2f}\n",
    s, i, f, d);
}
int main() {
  int mill = 1000*1000;                // 1 Million
  prints(mill, mill, mill, mill);      // short overflows
  // Output: short: 16960 int: 1000000 float: 1000000.00 double: 1000000.00
  long bill = 1000L*1000L*1000L*1000L; // 1 Trillion
  prints(bill, bill, bill, bill);      // even int overflows, float becomes inaccurate
  // Output: short: 4096 int: –727379968
  // float:999999995904.00 double: 1000000000000.00
  float three = 3.75f;
  prints(three, three, three, three);  // decimal places are lost
  // Output: short: 3 int: 3 float: 3.75 double: 3.75
}