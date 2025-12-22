//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Kx1KcP65T
#include <vector>
#include <iostream>
#include <cstdio> // fopen, fclose, fwrite, fread, remove
using namespace std;
ostream& operator<<(ostream&os, const vector<int>&data) {
    for(auto &e : data) os << e << ' '; return os;
}
static const char* FILENAME = "nums.dat";
int main() {
  const vector nums{10,11,22,34};
  { // write
    auto out = fopen(FILENAME, "wb"); // Open file with C for writing
    if(out==nullptr) {
        cerr << "Error opening file\n"; return -1;
    }
    auto ok = fwrite(nums.data(), sizeof(int), nums.size(), out);
    if(ok!=nums.size()) {
        cerr << "Error writing to file\n"; return -1;
    }
    fclose(out); // In C, you must explicitly close. Honestly.
  }
  vector<int> read{};
  { // read
    auto in = fopen(FILENAME, "rb"); // Open file with C for reading
    if(in==nullptr) {
        cerr << "Error opening file\n"; return -1;
    }
    const size_t sz = 4; // assumed, we know we are reading 4 elements …
    read.resize(sz);     // make space for data to be read
    auto ok = fread(read.data(), sizeof(int), sz, in);
    if(ok!=sz) {
        cerr << "Error reading\n"; return -1;
    }
    fclose(in);
  }
  { // Compare
    cout << nums << '\n';    // 10 11 22 34
    cout << read << '\n';    // 10 11 22 34
  }
  if(remove(FILENAME) == -1) {
      cerr << "Warning: Error deleting\n";
  }
}