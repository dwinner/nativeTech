
// https://godbolt.org/z/GE5v6Ea8x 
#include <map>
#include <string>
using std::map; using std::string;
struct MyClass {
  bool isFound(const map<int,string> &dict,  // immutable input parameter.
               const int &key,               // likewise
               string &result                // output parameter: not const
               ) const                       // instance of MyClass const
  {
    const map<int,string>::const_iterator where  // reference and value fixed
      = dict.find(key);
    if(where == end(dict)) {
      return false;
    } else {
      result = where->second;
      return true;
    }
  }
};