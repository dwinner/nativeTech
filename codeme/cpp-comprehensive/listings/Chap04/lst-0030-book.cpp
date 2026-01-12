
// https://godbolt.org/z/9943MfExc 
#include <vector> 
class Image { 
    std::vector<char> data_; 
public: 
    void load(const char* filename); // loads image data 
}; 
class Screen { 
public: 
    void show(Image& image);         // (ERR) image should be const 
}; 
void paint(Screen &screen, const Image& image) { 
    screen.show(image); 
} 
int main() { 
    Image image {}; 
    image.load("peter.png"); 
    Screen screen {}; 
    paint(screen, image); 
}