
// https://godbolt.org/z/EWqqGjnnz 
#include <iostream>  // cout, cerr
#include <fstream>
#include <vector>
#include <string>
using std::vector; using std::string; using std::cout; using std::cerr;

int countWords(const string& filename) {  // return negative on error
    std::ifstream file{filename};
    if(!file) {     // was there an error opening the file?
        cerr << "Error opening " << filename << "\n";
        return -1;  // report an error to the caller using a special value
    }
    int count = 0;
    string word;
    while(!file.eof()) {      // not at the end yet?
        file >> word;
        ++count;
    }
    return count-1;           // one more word was read at EOF
}
bool process(const vector<string>& args) { // return true if all okay
    if(args.size() == 0) {   // expecting parameters
        cerr << "Command line argument missing\n";
        return false;        // report an error via return
    } else {
        bool result = true;  // for the final result
        for(const string filename : args) {
            cout << filename << ": ";
            int count = countWords(filename);
            if(count < 0) {  // special return indicates error
                cout << "Error!\n";
                result = false;        // at least one error
            } else {
                cout << count << "\n"; // output normal result
            }
        }
        return result;                 // return overall result
    }
}

int main(int argc, const char* argv[]) {
    bool result = process(             // return value contains error indicator
        {argv+1, argv+argc} );         // const char*[] to vector<string>
    if(result) {                       // evaluate return value
        return 0;
    } else {
        cerr << "An error occurred.\n";
        return 1;                      // indicate error externally
    }
}