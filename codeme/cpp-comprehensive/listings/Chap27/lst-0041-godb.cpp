//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jddKxj7fW
#include <filesystem>   // std::filesystem
#include <iostream>
namespace fs = std::filesystem; using std::cout; using std::endl;
int main() {
    // Path components
    fs::path root {"/"};
    fs::path dir {"var/www/"};
    fs::path index {"index.html"};
    // concatenate
    fs::path p = root / dir / index;     // operator/
    // output
    cout << "Name: " << p << endl;       // "/var/www/index.html"
    // decompose
    cout << "Parent: " << p.parent_path() << endl;   // "/var/www"
    cout << "Name: " << p.filename() << endl;        // "index.html"
    cout << "Extension: " << p.extension() << endl;  // ".html"
    // Information
    cout << std::boolalpha;
    cout << "Exists? " << fs::exists(p) << endl;
    cout << "Regular file? " << fs::is_regular_file(p) << endl;
}