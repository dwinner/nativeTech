
// https://godbolt.org/z/ehhGPjabz 
#include <string>
#include <vector>
class Image {
    std::vector<char> imageData_;
public:
    explicit Image(const std::string& filename) { /* Load image */ }
    void draw() const { /* Paint image */ };
};