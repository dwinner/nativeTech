//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ehhGPjabz 
#include <string>
#include <vector>
class Image {
    std::vector<char> imageData_;
public:
    explicit Image(const std::string& filename) { /* Load image */ }
    void draw() const { /* Paint image */ };
};