//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9eeG575Wz 
struct Buffer {
  const char *data;
  explicit Buffer(unsigned sz): data(new char[sz]) {}
  ~Buffer() { delete[] data; }
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
};