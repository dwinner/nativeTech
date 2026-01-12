
// https://godbolt.org/z/E49on6zn3
#include <iostream>
#include <thread>
#include <vector>
#include <exception>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }

void main_program() {
    try {
        std::thread th{ &task1 };
        std::vector data{ 0,1,2 };
        data.at(666);                     // (ERR) triggers out_of_range
        th.join();                        // would wait
    } catch(std::runtime_error &ex) {     // (ERR) does not match out_of_range
         /*...*/
    }
}

int main() {
  try {
    main_program();
  } catch( ... ) {                        // so far, so good, looks safe
    std::cout << "An error has occurred\n"; // you won't see this
  }
}