//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a83sf9h8E 
#include <iostream>
// Production code:
struct DatabaseInterface {
    virtual int getData() const = 0;
};
struct Program {
    DatabaseInterface &db_;
    void run() {
        std:: cout << db_.getData() << "\n”;
    }
};
// Test helper:
struct MockDatabase : public DatabaseInterface {
    int getData() const override { return 5; }
};
// main as test:
int main() {
    MockDatabase mockDb;
    Program prog { mockDb }; // real DB is not tested
    prog.run();                // Expected output: 5
}