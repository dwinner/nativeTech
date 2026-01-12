
// https://godbolt.org/z/hG3j3PGYv (uses sqlite library)
#include <string>
#include <stdexcept>
#include <sqlite3.h>  // library
using std::string; using std::runtime_error;

void dbExec(const string &dbname, const string &sql) {
  sqlite3 *db;
  int errCode = sqlite3_open(dbname.c_str(), &db);  // acquire
  if(errCode) {
    throw runtime_error("Error opening the DB.");
  }
  errCode = sqlite3_exec(db, sql.c_str(), nullptr, nullptr, nullptr);
  if(errCode) {
    throw runtime_error("SQL Exec Error.");        // (ERR) not good!
  }
  errCode = sqlite3_close(db);                      // release
}