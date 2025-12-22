//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:sqlite@3400
// https://godbolt.org/z/c8xfM1nEa
#include <string>
#include <sqlite3.h>
class DbWrapper {
  sqlite3 *db_;
public:
  DbWrapper(const std::string& dbname)
    : db_{nullptr}
  {
    const int errCode = sqlite3_open(dbname.c_str(), &db_);
    if(errCode) db_ = nullptr;  // mark as 'not successful'
  }
  sqlite3* operator*() { return db_; }
  explicit operator bool() const {
    return db_ != nullptr;      // evaluate the mark
  }
  ~DbWrapper() {
    if(db_) sqlite3_close(db_);
  }
  // ... Rest as before ...
};
bool dbExec(const std::string &dbname, const std::string &sql) {
  DbWrapper db { dbname };
  if(db) {                      // check for successful initialization
    const int errCode = sqlite3_exec(*db,sql.c_str(),nullptr,nullptr,nullptr);
    if(errCode)
      return false;             // still correct RAII
  }
  return (bool)db;
}