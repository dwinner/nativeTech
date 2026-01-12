
// https://godbolt.org/z/9eeG575Wz 
struct Buffer {
  const char *data;
  explicit Buffer(unsigned sz): data(new char[sz]) {}
  ~Buffer() { delete[] data; }
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
};