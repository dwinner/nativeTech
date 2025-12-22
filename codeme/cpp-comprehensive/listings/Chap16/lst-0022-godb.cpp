//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/vb5a3EjcY 
#include <iostream>              // cout
#include "database.hpp"          // include the foreign API

class Database {
    const db_handle_t db_;                         // make constant
public:
    explicit Database(const char* filename);
    ~Database();
    int execute(const char* query);
    Database(const Database&) = delete;            // prohibit copying
    Database& operator=(const Database&) = delete; // prohibit assignment
};
// … Implementations as before …
int main() {
    Database db{ "customers.dat" };
    std::cout << "Count: "<< db.execute("select * from cust") << "\n";
    Database db2{ db };          // (ERR) compiler prevents dangerous copy
    db = db2;                    // (ERR) compiler prevents dangerous assignment
}