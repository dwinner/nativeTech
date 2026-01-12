
// https://godbolt.org/z/n54566cqT 
#include <iostream>
#include <typeinfo>
#include <string>
#include <map>
#include <boost/core/typeinfo.hpp>
int main() {
  using std::string; using std::cout;
  std::map<int, string> names;
  int i;
  double f;
  // demangled_name
  using boost::core::demangled_name;
  cout<<demangled_name(BOOST_CORE_TYPEID(i))<<'\n';      // Output: int
  cout<<demangled_name(BOOST_CORE_TYPEID(f))<<'\n';      // Output: double
  cout<<demangled_name(BOOST_CORE_TYPEID(string))<<'\n'; // Output: std::string
  cout<<demangled_name(BOOST_CORE_TYPEID(string{}.begin()))<<'\n';
  // Output: __gnu_cxx::__normal_iterator<char*, std::string>
  cout<<demangled_name(BOOST_CORE_TYPEID(namen))<<'\n';
  // Output: std::map<int, std::string, std::less<int>,
  //                                                         std::allocator<std::pair<int const, std::string> > >
  cout<<demangled_name(BOOST_CORE_TYPEID(666/0))<<'\n';  // Output: int
}