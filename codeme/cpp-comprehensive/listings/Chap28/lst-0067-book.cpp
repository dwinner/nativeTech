
// https://godbolt.org/z/1xnY8vd8Y 
#include <system_error> // error_code
#include <string>
void create_dir(const std::string& pathname, std::error_code& ec);
void run() {
  std::error_code ec;
  create_dir("/some/path", ec);
  if(!ec) {  // Success …
  } else {   // Failure …
  }
}