//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Y4sMdzde8
struct StereoImage {
    Image* right_;         // (ERR) raw pointer
    Image* left_;          // (ERR) raw pointer
    StereoImage(const string& nameBase)          // construct
      : right_{new Image{nameBase+"right.jpg"}}  // okay
      , left_{new Image{nameBase+"left.jpg"}}    // dangerous
      { }
    ~StereoImage() {       // remove
        delete right_; delete left_;
    }
    StereoImage(const StereoImage&) = delete;    // no copy
    StereoImage& operator=(const StereoImage&) = delete; // no assignment
};
int main() {
    Image* image = new Image{"image.jpg"};       // (ERR) a raw pointer?
    StereoImage stereo{"3d"};
    delete image;
}