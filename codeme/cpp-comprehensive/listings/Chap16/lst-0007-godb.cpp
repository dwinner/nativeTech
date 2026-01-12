//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/6399qK9Ts 
#include <iostream>                // cout
#include "database.hpp"            // include the foreign API
class Database {
    db_handle_t db_;               // wrapped resource
public:
    Database(const char* filename);
    ~Database();
    int execute(const char* query);
};
Database::Database(const char* filename)
    : db_{db_open(filename)}       // requesting the resource
    { }
Database::~Database() {
    db_close(db_);                 // releasing the resource
}
int Database::execute(const char* query) {
    return db_execute(db_, query); // using the resource
}
int main() {
    Database db{"customers.dat"};  // creating the wrapper
    std::cout << "Count: "<< db.execute("select * from cust") << "\n";
}                                  // automatically removing the wrapper