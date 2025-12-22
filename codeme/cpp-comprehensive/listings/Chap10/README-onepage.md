# Listings of Chap10.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 10.1: Respond to many states with different types of return values.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/EWqqGjnnz](https://godbolt.org/z/EWqqGjnnz):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
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
```

----------------


## Listing 10.2: Exceptions are triggered with "throw" and handled with "try-catch".

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/j8abxvvvo](https://godbolt.org/z/j8abxvvvo):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
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
```

----------------


## Listing 10.3: A "catch" can also be within a loop.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
void process(const vector<string>& args) {
    if(args.size() == 0) {                 // expect parameters
        throw std::invalid_argument{"Command line argument missing"};
    } else {
        for(const string filename : args) {
            cout << filename << ": ";
            try {
                cout << countWords(filename) << "\n";
            } catch(std::exception &exc) {
                cout << "Error: " << exc.what() << "\n";
            }
        }
    }
}
```

----------------


## Listing 10.4: Using "throw" without parameters rethrows the currently handled exception.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
try {
    cout << countWords(filename) << "\n";
} catch(std::exception &exc) {
    cout << "Error: " << exc.what() << "\n";
    throw; // rethrow
}
```

----------------


## Listing 10.5: You can also throw exceptions of other types.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/67cd5bhhE 
#include <string>
#include <iostream> // cout
using std::string; using std::to_string; using std::cout;
void triggerError(int errorCase) {
   try {
       if(errorCase < 10) throw (int)errorCase;
       else if(errorCase < 20) throw 1.0/(errorCase-10.0);
       else throw string{"Error " + to_string(errorCase)};
   } catch(int eval) {
       cout << "int-error: " << eval << "\n";
   } catch(double eval) {
       cout << "double-error: " << eval << "\n";
   } catch(string eval) {
       cout << "string-error: " << eval << "\n";
   }
}
int main() {
    triggerError(3);   // Output: int-error: 3
    triggerError(14);  // Output: double-error: 0.25
    triggerError(50);  // Output: string-error: Error 50
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/67cd5bhhE](https://godbolt.org/z/67cd5bhhE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/67cd5bhhE 
#include <string>
#include <iostream> // cout
using std::string; using std::to_string; using std::cout;
void triggerError(int errorCase) {
   try {
       if(errorCase < 10) throw (int)errorCase;
       else if(errorCase < 20) throw 1.0/(errorCase-10.0);
       else throw string{"Error " + to_string(errorCase)};
   } catch(int eval) {
       cout << "int-error: " << eval << "\n";
   } catch(double eval) {
       cout << "double-error: " << eval << "\n";
   } catch(string eval) {
       cout << "string-error: " << eval << "\n";
   }
}
int main() {
    triggerError(3);   // Output: int-error: 3
    triggerError(14);  // Output: double-error: 0.25
    triggerError(50);  // Output: string-error: Error 50
}
```

----------------


## Listing 10.6: At least output the error type and text instead of letting your program tumble out of "main".

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/TsP33njar 
#include <iostream>
#include <stdexcept> // exception

int main() {
    try {
        // … your other code …
    } catch(std::exception& exc) {
        std::cerr << "main: " << exc.what() << "\n";
    }
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/TsP33njar](https://godbolt.org/z/TsP33njar):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TsP33njar 
#include <iostream>
#include <stdexcept> // exception

int main() {
    try {
        // … your other code …
    } catch(std::exception& exc) {
        std::cerr << "main: " << exc.what() << "\n";
    }
}
```

----------------

