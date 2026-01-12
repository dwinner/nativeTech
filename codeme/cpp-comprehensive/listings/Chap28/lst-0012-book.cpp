
// https://godbolt.org/z/Yzefv59en
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_search; using std::cmatch;
int main() {
    cmatch res;                               // for detailed results
    std::string text = "<h2>Result and parts of it</h2>";
    regex pattern{"<h(.)>([^<]+)"};           // Search pattern with groups
    regex_search(text.c_str(), res, pattern); // Details to res
    std::cout << res[1] << ". "               // ()-Group 1: H-Level
         << res[2] << std::endl;              // ()-Group 2: H-Text
}