
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