
// https://godbolt.org/z/cs4eEEGrf 
#include <memory> // unique_ptr
struct Image {
    /* ... */
};
struct StereoImage {
  std::unique_ptr<Image> left, right;
  StereoImage()
  : left(new Image)
  , right(new Image)
  { }
};