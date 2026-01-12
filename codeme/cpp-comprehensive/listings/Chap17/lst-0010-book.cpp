
// https://godbolt.org/z/n7Grhh1GM
#include <sqlite3.h>
class DbWrapper {
  sqlite3 *db_;
public:
  // acquire resource
  DbWrapper(const string& dbname)
    : db_{nullptr}
  {
    const int errCode = sqlite3_open(dbname.c_str(), &db_);
    if(errCode)
      throw runtime_error("Error opening"); // prevents sqlite3_close
  }

  // release resource
  ~DbWrapper() {
    sqlite3_close(db_);                     // release
  }
  // access Resource
  sqlite3* operator*() { return db_; }
  // No copy and assignment
  DbWrapper(const DbWrapper&) = delete;
  DbWrapper& operator=(const DbWrapper&) = delete;
};
void dbExec(const string &dbname, const string &sql) {
  DbWrapper db { dbname };
  const int errCode = sqlite3_exec(*db, sql.c_str(), nullptr, 
    nullptr, nullptr);
  if(errCode)
    throw runtime_error("Error SQL-Exec."); // now it works!
}