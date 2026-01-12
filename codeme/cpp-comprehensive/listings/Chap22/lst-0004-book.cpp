
// https://godbolt.org/z/9Trh8h8Mb 
#include <string>
#include <vector>
#include <span>      // C++20
#include <fstream>   // ifstream
#include <stdexcept> // runtime_error
#include <iostream>  // cerr
// C-Header:
#include <zlib.h>    // gzXyz; sudo aptitude install libz-dev
#include <cerrno>    // errno
#include <cstring>   // strerror
namespace {
using std::string; using std::span; using std::byte;