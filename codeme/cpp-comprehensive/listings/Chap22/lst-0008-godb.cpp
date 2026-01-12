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