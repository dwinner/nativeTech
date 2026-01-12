
// https://godbolt.org/z/fsroE4seh 
#include <iostream>

// Production code:
struct DatabaseInterface {
    virtual int getData() const = 0;
};
struct RealDatabase : public DatabaseInterface {
    int getData() const override { return 999; }
};
struct Program {
    DatabaseInterface &db_;
    void run() {
        std:: cout << db_.getData() << "\n";
    }
};

// main as test:
int main() {
    RealDatabase db;
    Program prog { db }; // real DB is tested along
    prog.run();            // Expected output: 999
}