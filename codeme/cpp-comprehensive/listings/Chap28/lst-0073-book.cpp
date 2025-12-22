
// https://godbolt.org/z/n4v78WWod 
#include <system_error>
#include <string>

void create_dir(const std::string& pathname, std::error_code& ec) {
#if defined(_WIN32)
  // Windows implementation, with Windows error codes
#elif defined(linux)
  // Linux implementation, with Linux error codes
#else
  // general 'generic' case
  ec = std::make_error_code(std::errc::not_supported);
#endif
}