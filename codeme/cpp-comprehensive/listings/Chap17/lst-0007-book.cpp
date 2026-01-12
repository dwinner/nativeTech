
// https://godbolt.org/z/TK3vTv1EM 
struct StereoImage {
  Image *left, *right;
  StereoImage()
  : left(new Image)
  , right(new Image) // Danger!
  { }
  ~StereoImage() {
      delete left;
      delete right;
  }
};