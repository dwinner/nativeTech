# Listings of Chap22.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 22.1: The "gzpack.cpp" program uses a C library. Here you see the section with the includes.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/9Trh8h8Mb 
#include <string>
#include <vector>
#include <span>      // C++20
#include <fstream>   // ifstream
#include <stdexcept> // runtime_error
#include <iostream>  // cerr
// C-Header:
#include <zlib.h>    // gzXyz; sudo aptitude install libz-dev
#include <cerrno>    // errno
#include <cstring>   // strerror
namespace {
using std::string; using std::span; using std::byte;
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/9Trh8h8Mb](https://godbolt.org/z/9Trh8h8Mb):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9Trh8h8Mb 
#include <string>
#include <vector>
#include <span>      // C++20
#include <fstream>   // ifstream
#include <stdexcept> // runtime_error
#include <iostream>  // cerr
// C-Header:
#include <zlib.h>    // gzXyz; sudo aptitude install libz-dev
#include <cerrno>    // errno
#include <cstring>   // strerror
namespace {
using std::string; using std::span; using std::byte;
```

----------------


## Listing 22.2: This part of "gzpack.cpp" contains all the C functions used.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/Yh7j1WEGc 
class GzWriteStream {                         // RAII wrapper
public:
    gzFile gz_ ;                              // C struct from zlib.h
    explicit GzWriteStream(const string& filename)
        : gz_{gzopen(filename.c_str(),"wb9")} // 'w': write, 'b':binary, '9':level
    {
        if(gz_==NULL) throw std::runtime_error(std::strerror(errno));
    }
    ~GzWriteStream() {
        gzclose(gz_);
    }
    GzWriteStream& operator<<(const span<char> &data) {
        write(data);
        return *this;
    }
private:
    void write(span<char> data) {
        auto bytes = std::as_bytes(data);                    // C++20
        auto res = gzwrite(gz_, bytes.data(), size(bytes));
        if(res==0) throw std::runtime_error("Error writing");
    }
    GzWriteStream(const GzWriteStream&) = delete;            // no copy
    GzWriteStream& operator=(const GzWriteStream&) = delete; // no assignment
};
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/Yh7j1WEGc](https://godbolt.org/z/Yh7j1WEGc):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yh7j1WEGc 
class GzWriteStream {                         // RAII wrapper
public:
    gzFile gz_ ;                              // C struct from zlib.h
    explicit GzWriteStream(const string& filename)
        : gz_{gzopen(filename.c_str(),"wb9")} // 'w': write, 'b':binary, '9':level
    {
        if(gz_==NULL) throw std::runtime_error(std::strerror(errno));
    }
    ~GzWriteStream() {
        gzclose(gz_);
    }
    GzWriteStream& operator<<(const span<char> &data) {
        write(data);
        return *this;
    }
private:
    void write(span<char> data) {
        auto bytes = std::as_bytes(data);                    // C++20
        auto res = gzwrite(gz_, bytes.data(), size(bytes));
        if(res==0) throw std::runtime_error("Error writing");
    }
    GzWriteStream(const GzWriteStream&) = delete;            // no copy
    GzWriteStream& operator=(const GzWriteStream&) = delete; // no assignment
};
```

----------------


## Listing 22.3: This part of "gzpack.cpp" handles reading and writing the files.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/84TcTbWjj
std::vector<char> readFile(const string& fName) {
    std::ifstream file{ fName, std::ifstream::binary };
    if(!file) throw std::runtime_error("Error opening input");
    file.seekg(0, file.end);           // jump to the end of the file
    const auto length = file.tellg();  // current position is file size
    if(length > 1024*1024*1024)
        throw std::runtime_error("No more than 1 GB please");
    file.seekg(0, file.beg);           // back to the beginning
    std::vector<char> data(length);    // allocate space
    file.read(data.data(), length);    // read in one go
    return data;                       // not copied (keyword: RVO)
}
void pack(const string& fNameIn, const string& fNameOut) {
    auto data = readFile(fNameIn);     // read input
    GzWriteStream gz{fNameOut};        // initialize output
    gz << data;
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/84TcTbWjj](https://godbolt.org/z/84TcTbWjj):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/84TcTbWjj
std::vector<char> readFile(const string& fName) {
    std::ifstream file{ fName, std::ifstream::binary };
    if(!file) throw std::runtime_error("Error opening input");
    file.seekg(0, file.end);           // jump to the end of the file
    const auto length = file.tellg();  // current position is file size
    if(length > 1024*1024*1024)
        throw std::runtime_error("No more than 1 GB please");
    file.seekg(0, file.beg);           // back to the beginning
    std::vector<char> data(length);    // allocate space
    file.read(data.data(), length);    // read in one go
    return data;                       // not copied (keyword: RVO)
}
void pack(const string& fNameIn, const string& fNameOut) {
    auto data = readFile(fNameIn);     // read input
    GzWriteStream gz{fNameOut};        // initialize output
    gz << data;
}
```

----------------


## Listing 22.4: With "main" in "gzpack.cpp", the example is complete.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/5bWx6PzdP
} // namespace
int main(int argc, const char* argv[]) {
    try {
        const std::vector<string> fNamen {argv+1, argv+argc};
        for(auto fName : fNamen) {
            std::cout << "packing " << fName << "... ";
            pack(fName, fName+".gz");
            std::cout << fName << ".gz"<< "\n";
        }
    } catch(std::runtime_error &exc) {
        std::cerr << "Error: " << exc.what() << "\n";
    }
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/5bWx6PzdP](https://godbolt.org/z/5bWx6PzdP):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -lz; libs:-
// https://godbolt.org/z/5bWx6PzdP
} // namespace
int main(int argc, const char* argv[]) {
    try {
        const std::vector<string> fNamen {argv+1, argv+argc};
        for(auto fName : fNamen) {
            std::cout << "packing " << fName << "... ";
            pack(fName, fName+".gz");
            std::cout << fName << ".gz"<< "\n";
        }
    } catch(std::runtime_error &exc) {
        std::cerr << "Error: " << exc.what() << "\n";
    }
}
```

----------------

