//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TPWc5nexq 
#include <system_error> // error_code, errc
#include <string>
void create_dir(const std::string& pathname, std::error_code& ec);
void run() {
  std::error_code ec;
  create_dir("/some/path", ec);
  if(ec == std::errc::file_exists) {   // specifically …
  } else if(!ec) {                     // Success …
  } else {                             // Failure …
  }
}