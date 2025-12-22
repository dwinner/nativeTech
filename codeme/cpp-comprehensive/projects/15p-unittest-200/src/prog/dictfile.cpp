#include <cstdlib>
#include <iostream> // cout, cin
#include <fstream> // ifstream
#include <vector>
#include <string>
#include "qwort/qwort.hpp"

using std::cout; using std::cin;
using std::vector; using std::string;

constexpr size_t MIN_LINE_LEN = 8;

qw::index readIndexFromFile(std::istream &fin) {
    qw::index myindex{};
    // Index aus Datei bauen
    std::string line;
    while(std::getline(fin, line)) {
        if(line.length() < MIN_LINE_LEN)
            continue;
        myindex.add(line);
    }
    cout << "Read " << myindex.size() << " entries.\n";
    return myindex;
}

void queryIndexFromFile(const qw::index& index, std::istream &fin) {
    cout << "Now enter query strings, end with ^D (or ^Z on Windows).\n";
    std::string line;
    while(std::getline(cin, line)) {
        cout << "? " << line << "\n";
        cout << "= " << index.getBestMatch(line) << "\n";
    }
}

int main(int argc, const char* argv[]) {
    if(argc < 2) {
        cout << "Usage: " << argv[0] << " {dict-file} < {query-file}\n";
        return EXIT_FAILURE;
    }
    cout << "Reading file '" << argv[1] << "'...\n";
    std::ifstream fin{argv[1]};
    const auto index = readIndexFromFile(fin);
    queryIndexFromFile(index, cin);
    return EXIT_SUCCESS;
}
