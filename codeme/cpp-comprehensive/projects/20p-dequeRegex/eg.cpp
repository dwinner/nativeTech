#include <iostream>
#include <fstream>
#include "regexpr.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    if(argc != 2 && argc != 3) {
        cerr << "Usage: " << argv[0] << " pattern [file]" << endl;
        return 1;
    }
    ifstream in(0);
    if(argc == 3) {
        in.close();
        in.open(argv[2]);
        if(!in) {
            cerr << argv[0] << ": Can't open " << argv[2] << endl;
            return 1;
        }
    }
    RegExpr re;
    try {
        re.compile(argv[1]);
        int n;
        char buffer[500] = {};
        while(!in.eof()) {
            in.getline(buffer, 500);
            n = re.search(buffer);
            if(n >= 0)
                cout << buffer << endl;
        }
    }
    catch(bad_alloc exception) {
        cerr << endl << "Not enough memory." << endl;
        re.clear_after_error();
        return 1;
    }
    catch(xsyntax syntax) {
        cerr << endl << "Syntax error near character position "
             << syntax.getErrorPos() << endl;
        re.clear_after_error();
        return 1;
    }
    catch(...) {
        cerr << endl << "some other error." << endl;
        return 1;
    }
    return 0;
}
