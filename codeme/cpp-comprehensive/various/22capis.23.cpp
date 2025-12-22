//    #!cpp filename=22-capis-1.cpp test=save
#include <string>
#include <span>
#include <vector>
#include <fstream>   // ifstream
#include <stdexcept> // runtime_error
#include <iostream>  // cerr
// C-Header:
#include <zlib.h>    // gzXyz; sudo aptitude install libz-dev
#include <cerrno>    // errno
#include <cstring>   // strerror
#pragma GCC diagnostic ignored "-Wmissing-field-initializers" //#
namespace {
using std::string; using std::byte; using std::span;
//    #!cpp filename=22-capis-2.cpp test=save
class GzWriteStream {                         // RAII-Wrapper
public:
    gzFile gz_;                               // C-Struct from zlib.h

    explicit GzWriteStream(const string& filename)
        : gz_{gzopen(filename.c_str(),"wb9")} // 'w': write, 'b':binary, '9':level
    {
        if(gz_==NULL) throw std::runtime_error(std::strerror(errno));
    }

    ~GzWriteStream() {
        gzclose(gz_);
    }

    GzWriteStream& operator<<(const span<char> data) {
        write(data);
        return *this;
    }

private:
    void write(const span<char> data) {
        auto bytes = std::as_bytes(data);
        auto res = gzwrite(gz_, bytes.data(), size(bytes));
        if(res==0) throw std::runtime_error("Error writing file");
    }

    GzWriteStream(const GzWriteStream&) = delete;            // no copy
    GzWriteStream& operator=(const GzWriteStream&) = delete; // no assignment
};
//    #!cpp filename=22-capis-3.cpp test=save
std::vector<char> readFile(const string& fName) {
    std::ifstream file{ fName, std::ifstream::binary };
    if(!file) throw std::runtime_error("Error opening input");
    file.seekg(0, file.end);          // jump to end of file
    const auto length = file.tellg(); // current position is file size

    if(length > 1024*1024*1024)
        throw std::runtime_error("No more than 1 GB please");
    file.seekg(0, file.beg);          // back to the beginning
    std::vector<char> data(length);   // allocate space
    file.read(data.data(), length);   // read in one go
    return data;                      // not copied (RVO)
}
void pack(const string& fNameIn, const string& fNameOut) {
    auto data = readFile(fNameIn);    // read input
    GzWriteStream gz{fNameOut};       // initialize output
    gz << data;
}
//    #!cpp filename=22-capis-4.cpp testcxxflags=-lz test=skip
//    #include "46-capis-1.cpp" //#
//    #include "46-capis-2.cpp" //#
//    #include "46-capis-3.cpp" //#
} // namespace
int main(int argc, const char* argv[]) {
    try {
        const std::vector<string> fNames {argv+1, argv+argc};
        for(auto fName : fNames) {
            std::cout << "packing " << fName << "... ";
            pack(fName, fName+".gz");
            std::cout << fName << ".gz"<< "\n";
        }
    } catch(std::runtime_error &exc) {
        std::cerr << "Error: " << exc.what() << "\n";
    }
}