
// https://godbolt.org/z/c97qra3M7 
#include <vector>
#include <map>
#include <iostream>
using std::cout; using std::ostream;
template<typename K, typename T>
ostream& operator<<(ostream& os, std::pair<const K,T> value) {
    return os << '[' << value.first << ':' << value.second << ']';
}
int main() {
  {
    using Cont = std::vector<int>;
    Cont cont{ 1, 2, 3, 4, 5, 6 };
    Cont::size_type sz = cont.size();
    cout << "size=" << sz << " content= ";
    for(Cont::const_iterator it = cont.begin(); it != cont.end(); ++it) 
        cout << *it << ' ';
    cout << '\n';
  }
  {
    using Cont = std::map<int,char>;
    Cont cont{ {1,'a'}, {2,'b'}, {3,'c'}, {4,'d'}, {5,'e'}, {6,'f'} };
    Cont::size_type sz = cont.size();
    cout << "size=" << sz << " content= ";
    for(Cont::const_iterator it = cont.begin(); it != cont.end(); ++it) 
        cout << *it << ' ';
    cout << '\n';
  }
}