//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/zj91TshnY (includes all files)
#include <cstdlib>  // EXIT_SUCCESS xcv
#include <iostream> // cout
#include <vector>
#include <string>
#include "qwort/qwort.hpp"
using std::cout; using std::vector; using std::string;
int main(int argc, const char* argv[]) {
    cout << "qwort version " << qw::version() << "\n";

    /* Build index */
    qw::index myindex{};

    /* - Demo data */
    myindex.add("Germany");
    myindex.add("Greece");

    /* Generate queries */
    vector<string> args(argv+1, argv+argc); // iterator-based initialization
    for(auto &querystring : args) {
        cout << "Searching for '" << querystring << "'... ";
        const auto match = myindex.getBestMatch(querystring);
        cout << match << "\n";
    }
    return EXIT_SUCCESS;
}