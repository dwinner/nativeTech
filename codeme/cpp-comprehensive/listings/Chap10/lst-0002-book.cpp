
// https://godbolt.org/z/j8abxvvvo 
#include <iostream>            // cout, cerr
#include <vector>
#include <string>
#include <fstream>             // ifstream
#include <stdexcept>           // invalid_argument
using std::vector; using std::string; using std::cout; using std::ifstream;
size_t countWords(const string& filename) { // 0 or greater
    std::ifstream file{};      // create unopened
    // register for exceptions:
    file.exceptions(ifstream::failbit | ifstream::badbit);
    file.open(filename);       // could throw an exception
    size_t count = 0;
    string word;
    file.exceptions(ifstream::badbit); // EOF no longer an exception
    while(!file.eof()) {       // not at the end yet?
        file >> word;   ++count;
    }
    return count-1;            // one more word was read at EOF
}
void process(const vector<string>& args) {
    if(args.size() == 0) {     // process expects parameters
        throw std::invalid_argument{"Command line argument missing"}; // trigger
    } else {
        for(const string filename : args) {
            cout << filename << ": " << countWords(filename) << std::endl;
        }
    }
}
int main(int argc, const char* argv[]) {
    try {                                      // block with error handling
        process(
          vector<string>{argv+1, argv+argc} ); // const char*[] to vector<string>
        return 0;
    } catch(std::exception &exc) {             // error handling
        std::cerr << "An error occurred: " << exc.what() << "\n";
        return 1;
    }
}