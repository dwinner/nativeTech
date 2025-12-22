//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/75Kh5Ws45 
#include <vector>
class Image {
    std::vector<std::byte> data_;
public:
    explicit Image(const char *fn) { /*...*/ }
    // Compiler generates (among others): 
    // Copy constructor, assignment, but also moves
};
std::vector<Image> loadCollection(bool empty) {
    if(empty) return std::vector<Image>{};
    std::vector<Image> result {};                 // for return; initially empty
    // three images in the collection … copy?
    result.push_back( Image{"MonaLisa.png"} );
    result.push_back( Image{"TheScream.png"} );
    result.push_back( Image{"BoyWithPipe.png"} );
    return result; // return collection by value
}
int main() {
    // store return in variable
    std::vector<Image> collection = loadCollection(false);
}