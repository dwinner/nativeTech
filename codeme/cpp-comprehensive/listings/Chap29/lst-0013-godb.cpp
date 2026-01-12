//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/Pjzqsv9nz
// … includes …
#include <thread>
using namespace std::chrono; // seconds, suffix s
struct Image {
    std::vector<char> data_; // Copying expensive
    explicit Image() : data_(1'000'000) {}
};
void showImage(Image img) {
    std::cout << img.data_.size() << '\n';
}
void showIptr(std::unique_ptr<int> iptr) {
    std::cout << *iptr << '\n';
}
int main() {
    // expensive to copy, but good to move:
    Image image{};
    std::cout << image.data_.size() << std::endl;    // Output: 1000000
    std::jthread th1{ showImage, std::move(image) }; // Output: 1000000
    std::this_thread::sleep_for(1s);
    std::cout << image.data_.size() << std::endl;    // Output: 0
    th1.join();  // explicitly wait until the thread is done
    // impossible to copy, but good to move:
    auto iptr = std::make_unique<int>( 657 );
    std::cout << (bool)iptr << std::endl;            // Output: 1 for true
    std::jthread th2{ showIptr, std::move(iptr) };   // Output: 657
    std::this_thread::sleep_for(1s);
    std::cout << (bool)iptr.get() << std::endl;      // Output: 0 for false
}