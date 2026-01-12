
// https://godbolt.org/z/cjh93oPcr 
#include <iostream>
#include <system_error> // std::make_error_condition, std::ios_errc
int main () {
  // switch to exceptions:
  std::cin.exceptions (std::ios::failbit|std::ios::badbit);
  try {
    std::cin.rdbuf(nullptr);        // triggers an exception
  } catch (std::ios::failure& e) {  // derived from system_error
    std::cerr << "Error: ";
    if (e.code() == std::make_error_condition(std::io_errc::stream)) {
      std::cerr << "stream\n";
    } else {
      std::cerr << "other\n";
    }
  }
}