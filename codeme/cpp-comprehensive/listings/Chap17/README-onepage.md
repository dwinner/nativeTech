# Listings of Chap17.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 17.1: Prohibit four of the big five with "= delete".

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/K1714eWr9 
struct Type {
    char* data_;             // raw pointer can cause unclear ownership
    explicit Type(int n) : data_(new char[n]) {}
    ~Type() { delete[] data_; }             // you need the destructor

    Type(const Type&) = delete;             // do not allow copying
    Type& operator=(const Type&) = delete;  // no assignment please
    Type(Type&&) = delete;                  // no moving
    Type& operator=(Type&&) = delete;       // no move assignment operator
};
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/K1714eWr9](https://godbolt.org/z/K1714eWr9):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/K1714eWr9 
struct Type {
    char* data_;             // raw pointer can cause unclear ownership
    explicit Type(int n) : data_(new char[n]) {}
    ~Type() { delete[] data_; }             // you need the destructor

    Type(const Type&) = delete;             // do not allow copying
    Type& operator=(const Type&) = delete;  // no assignment please
    Type(Type&&) = delete;                  // no moving
    Type& operator=(Type&&) = delete;       // no move assignment operator
};
```

----------------


## Listing 17.2: Instead of using a raw pointer, use a standard construct and do not define any of the big five operations.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/nbh9aozPM 
#include <vector>
#include <memory>            // unique_ptr, shared_ptr
struct Type1 {               // automatic complete copy of the resource
    std::vector<char> data_;
    Type1(int n) : data_(n) {}
};
struct Type2 {               // copy prohibited, move allowed
    std::unique_ptr<int[]> data_;
    Type2(int n) : data_(new int[n]) {}
};
struct Type3 {               // copy allowed, resource is then cleanly shared
    std::shared_ptr<Type1> data_;
    Type3(int n) : data_(std::make_shared<Type1>(n)) {}
};
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/nbh9aozPM](https://godbolt.org/z/nbh9aozPM):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/nbh9aozPM 
#include <vector>
#include <memory>            // unique_ptr, shared_ptr
struct Type1 {               // automatic complete copy of the resource
    std::vector<char> data_;
    Type1(int n) : data_(n) {}
};
struct Type2 {               // copy prohibited, move allowed
    std::unique_ptr<int[]> data_;
    Type2(int n) : data_(new int[n]) {}
};
struct Type3 {               // copy allowed, resource is then cleanly shared
    std::shared_ptr<Type1> data_;
    Type3(int n) : data_(std::make_shared<Type1>(n)) {}
};
```

----------------


## Listing 17.3: In a hierarchy with virtual methods, you must define and mark the base class destructor as virtual.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/6vsqnWeWh 
struct Base {
    virtual ~Base() {};  // define the destructor, make it virtual
    virtual void other()
};

struct Derived : public Base {
    void other() override
};

int main() {
    Base *obj = new Derived{};
    /* ... more lines of code here ... */
    delete obj;          // works because Base::~Base is virtual
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/6vsqnWeWh](https://godbolt.org/z/6vsqnWeWh):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/6vsqnWeWh 
struct Base {
    virtual ~Base() {};  // define the destructor, make it virtual
    virtual void other()
};

struct Derived : public Base {
    void other() override
};

int main() {
    Base *obj = new Derived{};
    /* ... more lines of code here ... */
    delete obj;          // works because Base::~Base is virtual
}
```

----------------


## Listing 17.4: shared_ptr always calls the correct destructor, virtual or not.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
int main() {
    shared_ptr<Base> obj{ new Derived{} };
    /* ... more lines of code here ... */
} // obj is correctly cleaned up
```

----------------


## Listing 17.5: An RAII character buffer class.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/9eeG575Wz 
struct Buffer {
  const char *data;
  explicit Buffer(unsigned sz): data(new char[sz]) {}
  ~Buffer() { delete[] data; }
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
};
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/9eeG575Wz](https://godbolt.org/z/9eeG575Wz):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9eeG575Wz 
struct Buffer {
  const char *data;
  explicit Buffer(unsigned sz): data(new char[sz]) {}
  ~Buffer() { delete[] data; }
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
};
```

----------------


## Listing 17.6: If "right" throws an exception, a leak occurs with "left".

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/TK3vTv1EM 
struct StereoImage {
  Image *left, *right;
  StereoImage()
  : left(new Image)
  , right(new Image) // Danger!
  { }
  ~StereoImage() {
      delete left;
      delete right;
  }
};
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/TK3vTv1EM](https://godbolt.org/z/TK3vTv1EM):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TK3vTv1EM 
struct StereoImage {
  Image *left, *right;
  StereoImage()
  : left(new Image)
  , right(new Image) // Danger!
  { }
  ~StereoImage() {
      delete left;
      delete right;
  }
};
```

----------------


## Listing 17.7: Correct RAII for "StereoImage".

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/cs4eEEGrf 
#include <memory> // unique_ptr
struct Image {
    /* ... */
};
struct StereoImage {
  std::unique_ptr<Image> left, right;
  StereoImage()
  : left(new Image)
  , right(new Image)
  { }
};
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/cs4eEEGrf](https://godbolt.org/z/cs4eEEGrf):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cs4eEEGrf 
#include <memory> // unique_ptr
struct Image {
    /* ... */
};
struct StereoImage {
  std::unique_ptr<Image> left, right;
  StereoImage()
  : left(new Image)
  , right(new Image)
  { }
};
```

----------------


## Listing 17.8: C API in C++ Code

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/hG3j3PGYv](https://godbolt.org/z/hG3j3PGYv):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:sqlite@3400
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
```

----------------


## Listing 17.9: C API with simple RAII.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/n7Grhh1GM](https://godbolt.org/z/n7Grhh1GM):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:sqlite@3400
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
```

----------------


## Listing 17.10: C API with simple RAII, without throw.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/c8xfM1nEa](https://godbolt.org/z/c8xfM1nEa):
```cpp
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
```

----------------


## Listing 17.11: Nothrow-new does not throw "bad_alloc", but returns "nullptr".

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/s8KoE4ETs 
#include <new> // nothrow
std::string *ps = new(std::nothrow) std::string{};
if(ps == nullptr) {
   std::cerr << "Memory allocation failed\n";
   return SOME_ERROR;
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/s8KoE4ETs](https://godbolt.org/z/s8KoE4ETs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s8KoE4ETs 
#include <new> // nothrow
std::string *ps = new(std::nothrow) std::string{};
if(ps == nullptr) {
   std::cerr << "Memory allocation failed\n";
   return SOME_ERROR;
}
```

----------------

