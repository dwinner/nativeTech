
// https://godbolt.org/z/dG5vE3heM
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_search; using std::cmatch;
const regex pattern{R"(^score\s+(\d+)\s+for\s+(\d+)\s+nights?\s+at\s+(.*))"};
void extract(const std::string &text) {
    cmatch res;
    regex_search(text.c_str(), res, pattern);
    std::cout << res[1] << "," << res[2] << "," << res[3] << "\n";
}
int main() {
    extract("score 400 for 2 nights at Minas Tirith Airport");
    extract("score 84 for 1 night at Prancing Pony");
}