# Listings of Chap24.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 24.1: For parts of containers, use a pair of iteratos or if you use C++23, ranges.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/qGrYPrqYn 
#include <vector>
#include <iostream>
using namespace std;
int main() {
    vector nums{ 1,2,3 };
    vector four{ 4,5,6 };
    vector seven{ 7,8,9 } ;
    nums.insert(nums.begin(), four.begin(), four.end()); // pair of iterators
    cout << nums.size() << "\n";                         // Output: 9
    nums.insert_range(nums.begin(), seven);              // C++23: range
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/qGrYPrqYn](https://godbolt.org/z/qGrYPrqYn):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qGrYPrqYn 
#include <vector>
#include <iostream>
using namespace std;
int main() {
    vector nums{ 1,2,3 };
    vector four{ 4,5,6 };
    vector seven{ 7,8,9 } ;
    nums.insert(nums.begin(), four.begin(), four.end()); // pair of iterators
    cout << nums.size() << "\n";                         // Output: 9
    nums.insert_range(nums.begin(), seven);              // C++23: range
}
```

----------------


## Listing 24.2: A pair of iterators defines a range of elements.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/M1d9jEP74 
#include <vector>
#include <iostream>
using namespace std;
int main() {
    vector numbers{ 1,2,3,4,5 };
    numbers.erase(numbers.begin(), numbers.end());
    cout << numbers.size() << "\n"; // Output: 0
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/M1d9jEP74](https://godbolt.org/z/M1d9jEP74):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/M1d9jEP74 
#include <vector>
#include <iostream>
using namespace std;
int main() {
    vector numbers{ 1,2,3,4,5 };
    numbers.erase(numbers.begin(), numbers.end());
    cout << numbers.size() << "\n"; // Output: 0
}
```

----------------


## Listing 24.3: Indirectly uses begin() and end() of the container.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/Yv3vcxPv5 
#include <vector>
#include <iostream>
int main() {
    std::vector<int> numbers{ 1,2,3,4,5 };
    for(auto val : numbers) {
        std::cout << val << ' ';
    }
    std::cout << '\n';
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/Yv3vcxPv5](https://godbolt.org/z/Yv3vcxPv5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yv3vcxPv5 
#include <vector>
#include <iostream>
int main() {
    std::vector<int> numbers{ 1,2,3,4,5 };
    for(auto val : numbers) {
        std::cout << val << ' ';
    }
    std::cout << '\n';
}
```

----------------


## GodboltId:babK485oT

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/babK485oT 
#include <vector>
#include <iostream>
int main() {
    std::vector<int> numbers{ 1,2,3,4,5 };
    for(auto it = begin(numbers); it != end(numbers); ++it) {
        auto val = *it;
        std::cout << val << ' ';
    }
    std::cout << '\n';
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/babK485oT](https://godbolt.org/z/babK485oT):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/babK485oT 
#include <vector>
#include <iostream>
int main() {
    std::vector<int> numbers{ 1,2,3,4,5 };
    for(auto it = begin(numbers); it != end(numbers); ++it) {
        auto val = *it;
        std::cout << val << ' ';
    }
    std::cout << '\n';
}
```

----------------


## GodboltId:nj14e6Wq4

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/nj14e6Wq4 
#include <set>
#include <iostream>
int main() {
    std::set<int> numbers{ 10, 20, 90 };
    auto no = numbers.find(30);
    if(no == numbers.end()) { std::cout << "not there.\n"; }
    auto yes = numbers.find(20);
    if(yes != numbers.end()) { std::cout << *yes << '\n'; }
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/nj14e6Wq4](https://godbolt.org/z/nj14e6Wq4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nj14e6Wq4 
#include <set>
#include <iostream>
int main() {
    std::set<int> numbers{ 10, 20, 90 };
    auto no = numbers.find(30);
    if(no == numbers.end()) { std::cout << "not there.\n"; }
    auto yes = numbers.find(20);
    if(yes != numbers.end()) { std::cout << *yes << '\n'; }
}
```

----------------


## Listing 24.4: One algorithm after another with iterators.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/arfee3TTP 
using namespace std;
vector in { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
vector<int> tmp{};
vector<int> out{};
copy_if(in.begin(), in.end(), back_inserter(tmp), [](int i) { return i%3 == 0; });
transform(tmp.begin(), tmp.end(), back_inserter(out), [](int i) {return i*i; });
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/arfee3TTP](https://godbolt.org/z/arfee3TTP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/arfee3TTP 
using namespace std;
vector in { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
vector<int> tmp{};
vector<int> out{};
copy_if(in.begin(), in.end(), back_inserter(tmp), [](int i) { return i%3 == 0; });
transform(tmp.begin(), tmp.end(), back_inserter(out), [](int i) {return i*i; });
```

----------------


## Listing 24.5: A pipeline of views instead of algorithms.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/TG1qd31Mq 
using namespace std; namespace views = std::ranges::views;
vector in { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
auto out = in
    | views::filter([](int i) { return i%3 == 0; })
    | views::transform([](int i) {return i*i; });
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/TG1qd31Mq](https://godbolt.org/z/TG1qd31Mq):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TG1qd31Mq 
using namespace std; namespace views = std::ranges::views;
vector in { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
auto out = in
    | views::filter([](int i) { return i%3 == 0; })
    | views::transform([](int i) {return i*i; });
```

----------------


## Listing 24.8: For "const" objects, "begin()" and "end()" return a "const_iterator".

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/dxb98rWzo 
#include <iostream>                // cout
#include <vector>
using std::vector;

vector<int> createData(size_t sz) {
  return vector<int>(sz);          // sz x null
}
void fibonacci(vector<int> &data) {
  for(auto it = begin(data)+2; it != end(data); ++it) { // iterator it
    *it = *(it-1) + *(it-2);
  }
}

std::ostream& show(std::ostream &os, const vector<int> &data) {
  for(auto it=begin(data); it != end(data); ++it)      // const_iterator it
    std::cout << *it << " ";
  return os;
}

int main() {
  vector<int> data = createData(10);
  data[0] = 1;
  data[1] = 1;
  fibonacci(data);
  show(std::cout, data) << "\n";
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/dxb98rWzo](https://godbolt.org/z/dxb98rWzo):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dxb98rWzo 
#include <iostream>                // cout
#include <vector>
using std::vector;

vector<int> createData(size_t sz) {
  return vector<int>(sz);          // sz x null
}
void fibonacci(vector<int> &data) {
  for(auto it = begin(data)+2; it != end(data); ++it) { // iterator it
    *it = *(it-1) + *(it-2);
  }
}

std::ostream& show(std::ostream &os, const vector<int> &data) {
  for(auto it=begin(data); it != end(data); ++it)      // const_iterator it
    std::cout << *it << " ";
  return os;
}

int main() {
  vector<int> data = createData(10);
  data[0] = 1;
  data[1] = 1;
  fibonacci(data);
  show(std::cout, data) << "\n";
}
```

----------------


## Listing 24.9: Iterator adapters change the behavior of operations.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/7xYcqqh68 
#include <vector>
#include <iostream>   // cout
#include <iterator>   // ostream_iterator
#include <algorithm>  // copy

int main() {
  std::vector data { 1, 2, 3, 7, 9, 10 };
  std::ostream_iterator<int> out_it (std::cout,", "); // when assigned to cout
  std::copy(data.begin(), data.end(), out_it); // all elements to the iterator
  std::cout << "\n";                           // Output: 1, 2, 3, 7, 9, 10,
  // or from C++20 with Ranges:
  std::ranges::copy(data, out_it);
  std::cout << "\n";                           // Output: 1, 2, 3, 7, 9, 10,
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/7xYcqqh68](https://godbolt.org/z/7xYcqqh68):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7xYcqqh68 
#include <vector>
#include <iostream>   // cout
#include <iterator>   // ostream_iterator
#include <algorithm>  // copy

int main() {
  std::vector data { 1, 2, 3, 7, 9, 10 };
  std::ostream_iterator<int> out_it (std::cout,", "); // when assigned to cout
  std::copy(data.begin(), data.end(), out_it); // all elements to the iterator
  std::cout << "\n";                           // Output: 1, 2, 3, 7, 9, 10,
  // or from C++20 with Ranges:
  std::ranges::copy(data, out_it);
  std::cout << "\n";                           // Output: 1, 2, 3, 7, 9, 10,
}
```

----------------


## Listing 24.10: A (too) simple allocator and its usage.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/3dYhh6n5f
#include <set>
#include <vector>
#include <iostream>
template<class T> class GobAllocator  {
public:
    using value_type = T;
    T* allocate(size_t count) {
        size_t add = sizeof(T)*count;
        std::cout << "allocate("<<add<<"/"<<(buf_.size()-current_)<<")\n";
        if(current_+add > buf_.size()) throw std::bad_alloc{};
        char* result = buf_.data()+current_;
        current_ += add;
        return reinterpret_cast<T*>(result);
    }
    void deallocate(T* p, size_t count) {
        size_t del = sizeof(T)*count;
        std::cout << "deallocate("<<del<<")\n";
        if(del==current_ && p==reinterpret_cast<T*>(buf_.data())) {
            std::cout << "...all free.\n";
            current_ = 0;                // release everything again
        }
    }
    GobAllocator() : GobAllocator{1024} {}
    explicit GobAllocator(size_t mx)
      : buf_(mx, 0), current_{0} { }
private:
       std::vector<char> buf_;
       size_t current_;
};
int main() {
    constexpr size_t CNT = 1*1000*1000;
    using Gob = GobAllocator<int>;
    try {
        Gob gob(CNT*sizeof(int));        // prepare allocator
        std::vector<int,Gob> data(gob);
        data.reserve(CNT);               // get memory in one go
        for(int val=0; val < (int)CNT; ++val)
            data.push_back(val);
    } catch(std::bad_alloc &ex) {
        std::cout << "Memory exhausted.\n";
    }
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/3dYhh6n5f](https://godbolt.org/z/3dYhh6n5f):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/3dYhh6n5f
#include <set>
#include <vector>
#include <iostream>
template<class T> class GobAllocator  {
public:
    using value_type = T;
    T* allocate(size_t count) {
        size_t add = sizeof(T)*count;
        std::cout << "allocate("<<add<<"/"<<(buf_.size()-current_)<<")\n";
        if(current_+add > buf_.size()) throw std::bad_alloc{};
        char* result = buf_.data()+current_;
        current_ += add;
        return reinterpret_cast<T*>(result);
    }
    void deallocate(T* p, size_t count) {
        size_t del = sizeof(T)*count;
        std::cout << "deallocate("<<del<<")\n";
        if(del==current_ && p==reinterpret_cast<T*>(buf_.data())) {
            std::cout << "...all free.\n";
            current_ = 0;                // release everything again
        }
    }
    GobAllocator() : GobAllocator{1024} {}
    explicit GobAllocator(size_t mx)
      : buf_(mx, 0), current_{0} { }
private:
       std::vector<char> buf_;
       size_t current_;
};
int main() {
    constexpr size_t CNT = 1*1000*1000;
    using Gob = GobAllocator<int>;
    try {
        Gob gob(CNT*sizeof(int));        // prepare allocator
        std::vector<int,Gob> data(gob);
        data.reserve(CNT);               // get memory in one go
        for(int val=0; val < (int)CNT; ++val)
            data.push_back(val);
    } catch(std::bad_alloc &ex) {
        std::cout << "Memory exhausted.\n";
    }
}
```

----------------


## Listing 24.11: Type aliases are sometimes clearer than the concrete types.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/c97qra3M7](https://godbolt.org/z/c97qra3M7):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
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
```

----------------


## Listing 24.12: This is the template for the example listings in this section for vector.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/WK86eaz1x 
#include <vector>
#include <iostream>
using std::vector; using std::cout;
template<typename T>
std::ostream& operator<<(std::ostream&os, const vector<T>& data) {
    for(const auto &e : data) {
        os << e << ' ';
    }
    return os;
}
int main() {h
   // Example code here
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/WK86eaz1x](https://godbolt.org/z/WK86eaz1x):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/WK86eaz1x 
#include <vector>
#include <iostream>
using std::vector; using std::cout;
template<typename T>
std::ostream& operator<<(std::ostream&os, const vector<T>& data) {
    for(const auto &e : data) {
        os << e << ' ';
    }
    return os;
}
int main() {h
   // Example code here
}
```

----------------


## Listing 24.13: The default constructor initializes an empty vector.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/7zMP5fo7c 
vector<int> dataA;
vector<int> dataB{};
vector<int> dataC = {};   // no assignment
cout << format("{} {} {}\n", dataA.size(), dataB.size(), dataC.size()); // 0 0 0
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/7zMP5fo7c](https://godbolt.org/z/7zMP5fo7c):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7zMP5fo7c 
vector<int> dataA;
vector<int> dataB{};
vector<int> dataC = {};   // no assignment
cout << format("{} {} {}\n", dataA.size(), dataB.size(), dataC.size()); // 0 0 0
```

----------------


## Listing 24.14: Copying a vector using the constructor or implicitly.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/cE6G1aErs 
auto count(vector<int> arg) { return arg.size(); }
// …
vector input{1,2,3};           // vector<int>
vector outputA(input);         // copy
vector outputB = input;        // also copy, no assignment
cout << count(input) << '\n';  // call by copy-by-value
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/cE6G1aErs](https://godbolt.org/z/cE6G1aErs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cE6G1aErs 
auto count(vector<int> arg) { return arg.size(); }
// …
vector input{1,2,3};           // vector<int>
vector outputA(input);         // copy
vector outputB = input;        // also copy, no assignment
cout << count(input) << '\n';  // call by copy-by-value
```

----------------


## Listing 24.15: For return values, the compiler can often also move.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/fv5WEe97M
#include <list>
#include <string>
#include <iterator>                     // make_move_iterator
using std::make_move_iterator; using std::string;
vector<int> create() { return vector<int>{8, 9, 10}; }
size_t count(vector<int> d) { return d.size(); }
// …
vector input{1,2,3};
vector outputA(std::move(input));       // move
vector outputB = std::move(input);      // also move, not assignment
vector data = create();                 // Return-Value-Optimization
cout << count(input) << '\n';           // call by Copy-by-Value
// move elements from another container
std::list<string> source{ "a", "a", "a", "BB", "CC", "DD", "b", "b" };
auto from = source.begin();
std::advance(from, 3); // 3 forward, but slow in list
auto to = from;
std::advance(to, 3);   // another 3 forward
vector target(make_move_iterator(from), make_move_iterator(to));
// source is now {"a", "a", "a", "", "", "", "b", "b"}, target is now {"BB", "CC", "DD"}
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/fv5WEe97M](https://godbolt.org/z/fv5WEe97M):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/fv5WEe97M
#include <list>
#include <string>
#include <iterator>                     // make_move_iterator
using std::make_move_iterator; using std::string;
vector<int> create() { return vector<int>{8, 9, 10}; }
size_t count(vector<int> d) { return d.size(); }
// …
vector input{1,2,3};
vector outputA(std::move(input));       // move
vector outputB = std::move(input);      // also move, not assignment
vector data = create();                 // Return-Value-Optimization
cout << count(input) << '\n';           // call by Copy-by-Value
// move elements from another container
std::list<string> source{ "a", "a", "a", "BB", "CC", "DD", "b", "b" };
auto from = source.begin();
std::advance(from, 3); // 3 forward, but slow in list
auto to = from;
std::advance(to, 3);   // another 3 forward
vector target(make_move_iterator(from), make_move_iterator(to));
// source is now {"a", "a", "a", "", "", "", "b", "b"}, target is now {"BB", "CC", "DD"}
```

----------------


## Listing 24.16: Using an initializer list to prefill a vector. Pay attention to the correct types in the list.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
// https://godbolt.org/z/6rG393Wa5
vector<int> primes{ 2,3,5,7,11 };
vector evens{ 2,4,6,8,10 };
vector<int> notLikeThis{ 'a', 4.3, 8L }; // (ERR) "Narrowing" double not okay
vector<string> names{ "are", "only" };   // converts arguments
vector sound{ "smoke", "fume" };         // dangerous: vector<const char[]>
vector wet{ "rain"s, "water"s };         // vector<string>
vector cold{ "ice"sv, "pole"sv };        // vector<string_view>
```

Godbolt Listing [lst-0027-godb.cpp](lst-0027-godb.cpp), [https://godbolt.org/z/6rG393Wa5](https://godbolt.org/z/6rG393Wa5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6rG393Wa5
vector<int> primes{ 2,3,5,7,11 };
vector evens{ 2,4,6,8,10 };
vector<int> notLikeThis{ 'a', 4.3, 8L }; // (ERR) "Narrowing" double not okay
vector<string> names{ "are", "only" };   // converts arguments
vector sound{ "smoke", "fume" };         // dangerous: vector<const char[]>
vector wet{ "rain"s, "water"s };         // vector<string>
vector cold{ "ice"sv, "pole"sv };        // vector<string_view>
```

----------------


## Listing 24.17: Copy values from any other container or C-array during initialization.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
// https://godbolt.org/z/h8Pze74ME
#include <deque>
#include <ranges>                       // C++20
// …
std::deque in{1,2,33,34,35,99};
vector thirty(in.begin()+2, in.begin()+5);
for(auto &e : thirty) {
    cout << e << ' ';
}
cout << '\n';
namespace vs = std::ranges::views;      // C++20
auto v = in | vs::drop(2) | vs::take(3);
vector otuz(v.begin(), v.end());
vector trente(std::from_range, in);     // C++23
```

Godbolt Listing [lst-0028-godb.cpp](lst-0028-godb.cpp), [https://godbolt.org/z/h8Pze74ME](https://godbolt.org/z/h8Pze74ME):
```cpp
//#(compile) c++; compiler:gsnapshot; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/h8Pze74ME
#include <deque>
#include <ranges>                       // C++20
// …
std::deque in{1,2,33,34,35,99};
vector thirty(in.begin()+2, in.begin()+5);
for(auto &e : thirty) {
    cout << e << ' ';
}
cout << '\n';
namespace vs = std::ranges::views;      // C++20
auto v = in | vs::drop(2) | vs::take(3);
vector otuz(v.begin(), v.end());
vector trente(std::from_range, in);     // C++23
```

----------------


## Listing 24.18: Preinitializing with a fixed number of values is (almost) only available with "vector".

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/addobjfsK 
vector<int> zeros(10);        // 10 zeros
vector<int> sixes(10, 6);     // 10 sixes
vector<int> ten{10};          // (ERR) Attention! Only one 10
vector<int> tenSix{10, 6};    // (ERR) Attention! Two elements 10 and 6
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/addobjfsK](https://godbolt.org/z/addobjfsK):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/addobjfsK 
vector<int> zeros(10);        // 10 zeros
vector<int> sixes(10, 6);     // 10 sixes
vector<int> ten{10};          // (ERR) Attention! Only one 10
vector<int> tenSix{10, 6};    // (ERR) Attention! Two elements 10 and 6
```

----------------


## Listing 24.19: You can assign to "vector" or reinitialize it later with "assign".

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/Yb4jEdaKj 
vector<int> from{ 2,3,4 };
vector<int> to{};
to = from;                  // Assignment with operator=, now both are the same

vector<int> drain{};
sink = std::move(from);           // Move, now 'from' is empty
vector<int> v;
v.assign(4, 100);                 // v is now {100, 100, 100, 100}
v.assign(to.begin(), to.end());   // v is now {2,3,4}
int z[] = { 10, 20, 30, 40 };
v.assign(z+1, z+4);               // v is now {20, 30, 40}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/Yb4jEdaKj](https://godbolt.org/z/Yb4jEdaKj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yb4jEdaKj 
vector<int> from{ 2,3,4 };
vector<int> to{};
to = from;                  // Assignment with operator=, now both are the same

vector<int> drain{};
sink = std::move(from);           // Move, now 'from' is empty
vector<int> v;
v.assign(4, 100);                 // v is now {100, 100, 100, 100}
v.assign(to.begin(), to.end());   // v is now {2,3,4}
int z[] = { 10, 20, 30, 40 };
v.assign(z+1, z+4);               // v is now {20, 30, 40}
```

----------------


## Listing 24.20: With "begin", "end", and their relatives, you get iterators.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// https://godbolt.org/z/freaK1Pea
vector vowels { 'A', 'e', 'i', 'o', 'u' };
const vector even { '0', '2', '4', '6', '8' };
auto it1 = vowels.begin();           // vector<char>::iterator
*it1 = 'a';                          // '*it1' returns 'char&'
auto it2 = even.begin();             // vector<char>::const_iterator
auto it3 = vowels.cbegin();          // enforces const_iterator
*i2 = '9'; *i3 = 'x';                // (ERR) 'const char&' is not modifiable
for(auto it=vowels.cbegin()+1; it!=vowels.cend(); ++it)
    { cout << *it; } cout << '\n';   // Output: eiou
for(auto it=vowels.crbegin()+1; it!=vowels.crend(); ++it) // ++ despite reverse!
    { cout << *it; } cout << '\n';   // Output: oiea
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/freaK1Pea](https://godbolt.org/z/freaK1Pea):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/freaK1Pea
vector vowels { 'A', 'e', 'i', 'o', 'u' };
const vector even { '0', '2', '4', '6', '8' };
auto it1 = vowels.begin();           // vector<char>::iterator
*it1 = 'a';                          // '*it1' returns 'char&'
auto it2 = even.begin();             // vector<char>::const_iterator
auto it3 = vowels.cbegin();          // enforces const_iterator
*i2 = '9'; *i3 = 'x';                // (ERR) 'const char&' is not modifiable
for(auto it=vowels.cbegin()+1; it!=vowels.cend(); ++it)
    { cout << *it; } cout << '\n';   // Output: eiou
for(auto it=vowels.crbegin()+1; it!=vowels.crend(); ++it) // ++ despite reverse!
    { cout << *it; } cout << '\n';   // Output: oiea
```

----------------


## Listing 24.21: With "vector" iterators, you can perform random access.

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/srczx737G
#include <vector>
#include <iostream>
#include <algorithm>               // ranges::sort
using std::vector; using std::cout;
double median(vector<int> data) {  // copied
    std::ranges::sort(data);       // C++20, otherwise std::sort()
    auto it = data.begin();
    auto sz = data.size();
    if(sz==0) return 0;            // special case
    // Determine median:
    auto m = (it+sz/2);            // approximately the middle
    if(sz%2 != 0) {                // odd number of elements
        return *m;
    } else {                       // even number of elements
        return double(*m + *(m-1)) / 2;
    }
}
int main() {
    vector data1 { 12, 22, 34, 10, 1, 99, 33 };
    cout << median(data1) << '\n'; // 22
    vector data2 { 30, 2, 80, 99, 31, 3 };
    cout << median(data2) << '\n'; // 30.5
}
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/srczx737G](https://godbolt.org/z/srczx737G):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/srczx737G
#include <vector>
#include <iostream>
#include <algorithm>               // ranges::sort
using std::vector; using std::cout;
double median(vector<int> data) {  // copied
    std::ranges::sort(data);       // C++20, otherwise std::sort()
    auto it = data.begin();
    auto sz = data.size();
    if(sz==0) return 0;            // special case
    // Determine median:
    auto m = (it+sz/2);            // approximately the middle
    if(sz%2 != 0) {                // odd number of elements
        return *m;
    } else {                       // even number of elements
        return double(*m + *(m-1)) / 2;
    }
}
int main() {
    vector data1 { 12, 22, 34, 10, 1, 99, 33 };
    cout << median(data1) << '\n'; // 22
    vector data2 { 30, 2, 80, 99, 31, 3 };
    cout << median(data2) << '\n'; // 30.5
}
```

----------------


## Listing 24.22: The normal case is access with [], because you already check the boundaries elsewhere.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/jjWx3d375
vector d{ 1, 2, 4, -1, 1, 2, -2 };
for(size_t idx=0; idx < d.size(); ) {  // checks vector boundary
    cout << d[idx] << ' ';             // additional check with at not necessary
    idx += d[idx];                     // same here
}
cout << '\n';
// Output: 1 2 -1 4 -2 1 2
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/jjWx3d375](https://godbolt.org/z/jjWx3d375):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jjWx3d375
vector d{ 1, 2, 4, -1, 1, 2, -2 };
for(size_t idx=0; idx < d.size(); ) {  // checks vector boundary
    cout << d[idx] << ' ';             // additional check with at not necessary
    idx += d[idx];                     // same here
}
cout << '\n';
// Output: 1 2 -1 4 -2 1 2
```

----------------


## Listing 24.23: Constant container returns constant reference.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/jGx51caTj 
#include <vector>
#include <iostream>
void printAndMore(const std::vector<int>& data) { // by-const-ref
    std::cout << data[0] << std::endl;
    data[0] = 666;         // (ERR) doesn't work because 'const int&'
}
int main() {
    std::vector numbers {1,2,3};
    printAndMore(numbers);
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/jGx51caTj](https://godbolt.org/z/jGx51caTj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jGx51caTj 
#include <vector>
#include <iostream>
void printAndMore(const std::vector<int>& data) { // by-const-ref
    std::cout << data[0] << std::endl;
    data[0] = 666;         // (ERR) doesn't work because 'const int&'
}
int main() {
    std::vector numbers {1,2,3};
    printAndMore(numbers);
}
```

----------------


## Listing 24.24: insert shifts all elements back by one here.

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
// https://godbolt.org/z/s9P1Y6jhv
vector<string> cars{ "Diesel", "Petrol", "Super", "Gas" };
cout << cars[1] << '\n';                   // Output: Petrol
cars.insert(cars.begin(), "Electricity");  // shifts everything back by one
cout << cars[1] << '\n';                   // Output: Diesel
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/s9P1Y6jhv](https://godbolt.org/z/s9P1Y6jhv):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/s9P1Y6jhv
vector<string> cars{ "Diesel", "Petrol", "Super", "Gas" };
cout << cars[1] << '\n';                   // Output: Petrol
cars.insert(cars.begin(), "Electricity");  // shifts everything back by one
cout << cars[1] << '\n';                   // Output: Diesel
```

----------------


## Listing 24.25: Use data() as an interface to C.

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
// https://godbolt.org/z/Kx1KcP65T
#include <vector>
#include <iostream>
#include <cstdio> // fopen, fclose, fwrite, fread, remove
using namespace std;
ostream& operator<<(ostream&os, const vector<int>&data) {
    for(auto &e : data) os << e << ' '; return os;
}
static const char* FILENAME = "nums.dat";
int main() {
  const vector nums{10,11,22,34};
  { // write
    auto out = fopen(FILENAME, "wb"); // Open file with C for writing
    if(out==nullptr) {
        cerr << "Error opening file\n"; return -1;
    }
    auto ok = fwrite(nums.data(), sizeof(int), nums.size(), out);
    if(ok!=nums.size()) {
        cerr << "Error writing to file\n"; return -1;
    }
    fclose(out); // In C, you must explicitly close. Honestly.
  }
  vector<int> read{};
  { // read
    auto in = fopen(FILENAME, "rb"); // Open file with C for reading
    if(in==nullptr) {
        cerr << "Error opening file\n"; return -1;
    }
    const size_t sz = 4; // assumed, we know we are reading 4 elements …
    read.resize(sz);     // make space for data to be read
    auto ok = fread(read.data(), sizeof(int), sz, in);
    if(ok!=sz) {
        cerr << "Error reading\n"; return -1;
    }
    fclose(in);
  }
  { // Compare
    cout << nums << '\n';    // 10 11 22 34
    cout << read << '\n';    // 10 11 22 34
  }
  if(remove(FILENAME) == -1) {
      cerr << "Warning: Error deleting\n";
  }
}
```

Godbolt Listing [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/Kx1KcP65T](https://godbolt.org/z/Kx1KcP65T):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Kx1KcP65T
#include <vector>
#include <iostream>
#include <cstdio> // fopen, fclose, fwrite, fread, remove
using namespace std;
ostream& operator<<(ostream&os, const vector<int>&data) {
    for(auto &e : data) os << e << ' '; return os;
}
static const char* FILENAME = "nums.dat";
int main() {
  const vector nums{10,11,22,34};
  { // write
    auto out = fopen(FILENAME, "wb"); // Open file with C for writing
    if(out==nullptr) {
        cerr << "Error opening file\n"; return -1;
    }
    auto ok = fwrite(nums.data(), sizeof(int), nums.size(), out);
    if(ok!=nums.size()) {
        cerr << "Error writing to file\n"; return -1;
    }
    fclose(out); // In C, you must explicitly close. Honestly.
  }
  vector<int> read{};
  { // read
    auto in = fopen(FILENAME, "rb"); // Open file with C for reading
    if(in==nullptr) {
        cerr << "Error opening file\n"; return -1;
    }
    const size_t sz = 4; // assumed, we know we are reading 4 elements …
    read.resize(sz);     // make space for data to be read
    auto ok = fread(read.data(), sizeof(int), sz, in);
    if(ok!=sz) {
        cerr << "Error reading\n"; return -1;
    }
    fclose(in);
  }
  { // Compare
    cout << nums << '\n';    // 10 11 22 34
    cout << read << '\n';    // 10 11 22 34
  }
  if(remove(FILENAME) == -1) {
      cerr << "Warning: Error deleting\n";
  }
}
```

----------------


## Listing 24.26: How to use span.

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
// https://godbolt.org/z/h7jf4KM1q 
#include <vector>
#include <span>
#include <iostream>
using namespace std;
int sum(span<const int> area) {  // C++20: span
   int sum = 0;
   for(auto e : range) {         // algorithm would be better …
    sum += e;
   }
   return sum;
}
int main() {
    vector data {1,2,3,4,5};
    cout << sum(data) << "\n";   // implicit container to span
    auto [b, e, sz] = make_tuple(begin(data), end(data), size(data));
    cout << sum(span{b,   sz-1}) << "\n"; // 1..4 (Iter, size)
    cout << sum(span{b+1, sz-1}) << "\n"; // 2..5 (Iter, size)
    cout << sum(span{b,   e-2}) << "\n";  // 1..3 (Iter, Iter)
    cout << sum(span{b+1, e-1}) << "\n";  // 2..4 (Iter, Iter)
    cout << sum(span{b+2, e  }) << "\n";  // 3..5 (Iter, Iter)
    return 0;
}
```

Godbolt Listing [lst-0041-godb.cpp](lst-0041-godb.cpp), [https://godbolt.org/z/h7jf4KM1q](https://godbolt.org/z/h7jf4KM1q):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h7jf4KM1q 
#include <vector>
#include <span>
#include <iostream>
using namespace std;
int sum(span<const int> area) {  // C++20: span
   int sum = 0;
   for(auto e : range) {         // algorithm would be better …
    sum += e;
   }
   return sum;
}
int main() {
    vector data {1,2,3,4,5};
    cout << sum(data) << "\n";   // implicit container to span
    auto [b, e, sz] = make_tuple(begin(data), end(data), size(data));
    cout << sum(span{b,   sz-1}) << "\n"; // 1..4 (Iter, size)
    cout << sum(span{b+1, sz-1}) << "\n"; // 2..5 (Iter, size)
    cout << sum(span{b,   e-2}) << "\n";  // 1..3 (Iter, Iter)
    cout << sum(span{b+1, e-1}) << "\n";  // 2..4 (Iter, Iter)
    cout << sum(span{b+2, e  }) << "\n";  // 3..5 (Iter, Iter)
    return 0;
}
```

----------------


## Listing 24.27: These are spans with fixed sizes.

Book listing [lst-0044-book.cpp](lst-0044-book.cpp):
```cpp
// https://godbolt.org/z/a3qqr1YPM 
int car[5]  {1,2,3,4,5};
span span_1 = car;           // directly from a C-array
array arr   {1,2,3,4,5};
span span_2 {arr};           // directly from a std::array
vector vec  {1,2,3,4,5};
span<int,3> span_3 {vec};    // with 'Extent' from a std::vector
```

Godbolt Listing [lst-0044-godb.cpp](lst-0044-godb.cpp), [https://godbolt.org/z/a3qqr1YPM](https://godbolt.org/z/a3qqr1YPM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a3qqr1YPM 
int car[5]  {1,2,3,4,5};
span span_1 = car;           // directly from a C-array
array arr   {1,2,3,4,5};
span span_2 {arr};           // directly from a std::array
vector vec  {1,2,3,4,5};
span<int,3> span_3 {vec};    // with 'Extent' from a std::vector
```

----------------


## Listing 24.28: You can also write through a "span".

Book listing [lst-0046-book.cpp](lst-0046-book.cpp):
```cpp
// https://godbolt.org/z/6Ez5GdozG 
#include <vector>
#include <span>
#include <iostream>
using namespace std;
void inc(span<int> span) {
   for(auto& e :span) {  // Reference
     e += 1;             // write
   }
}
int main() {
    vector data {1,2,3,4,5};
    span whole{data};              //    1,2,3,4,5
    inc(whole.first(3));           // -> 2,3,4,4,5
    inc(whole.last(3));            // -> 2,3,5,5,6
    inc(whole.last(4).first(3));   // -> 2,4,6,6,6
    inc(whole.subspan(1,3));       // -> 2,5,7,7,6
    for(auto i: whole) cout << i << ' '; cout << '\n'; // Output: 2 5 7 7 6
    return 0;
}
```

Godbolt Listing [lst-0046-godb.cpp](lst-0046-godb.cpp), [https://godbolt.org/z/6Ez5GdozG](https://godbolt.org/z/6Ez5GdozG):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6Ez5GdozG 
#include <vector>
#include <span>
#include <iostream>
using namespace std;
void inc(span<int> span) {
   for(auto& e :span) {  // Reference
     e += 1;             // write
   }
}
int main() {
    vector data {1,2,3,4,5};
    span whole{data};              //    1,2,3,4,5
    inc(whole.first(3));           // -> 2,3,4,4,5
    inc(whole.last(3));            // -> 2,3,5,5,6
    inc(whole.last(4).first(3));   // -> 2,4,6,6,6
    inc(whole.subspan(1,3));       // -> 2,5,7,7,6
    for(auto i: whole) cout << i << ' '; cout << '\n'; // Output: 2 5 7 7 6
    return 0;
}
```

----------------


## Listing 24.29: This is how the "mdspan" added in C++23 works.

Book listing [lst-0047-book.cpp](lst-0047-book.cpp):
```cpp
// https://godbolt.org/z/n8jn7v5vb 
#include <mdspan>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    // 1D: 12 elements
    vector v{1,2,3,4,5,6,7,8,9,10,11,12};
    // 2D: as 2 rows with 6 ints each
    auto ms2 = mdspan(v.data(), 2, 6);
    // 3D: as a cuboid with 2 layers, 3 rows, 2 columns
    auto ms3 = mdspan(v.data(), 2, 3, 2);
    // write via 2D view
    for (auto i = 0; i != ms2.extent(0); ++i)
      for (auto j = 0; j != ms2.extent(1); ++j)
        ms2[i, j] = i * 100 + j;  // write via multidimensional index
    // read via 3D view
    for (auto i = 0; i != ms3.extent(0); ++i) {
      cout << "Level " << i << ":\n";
      for (auto j = 0; j != ms3.extent(1); ++j) {
        for (auto k = 0; k != ms3.extent(2); ++k)
          cout << " " << ms3[i, j, k];  // read via multidimensional index
        cout << '\n';
      }
    }
    // Output: Level 0: 0 1, 2 3, 4 5, Level 1: 100 101, 102 103, 104 105
}
```

Godbolt Listing [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/n8jn7v5vb](https://godbolt.org/z/n8jn7v5vb):
```cpp
//#(compile) c++; compiler:gsnapshot; options:"-std=c++23"; libs:-
// https://godbolt.org/z/n8jn7v5vb 
#include <mdspan>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    // 1D: 12 elements
    vector v{1,2,3,4,5,6,7,8,9,10,11,12};
    // 2D: as 2 rows with 6 ints each
    auto ms2 = mdspan(v.data(), 2, 6);
    // 3D: as a cuboid with 2 layers, 3 rows, 2 columns
    auto ms3 = mdspan(v.data(), 2, 3, 2);
    // write via 2D view
    for (auto i = 0; i != ms2.extent(0); ++i)
      for (auto j = 0; j != ms2.extent(1); ++j)
        ms2[i, j] = i * 100 + j;  // write via multidimensional index
    // read via 3D view
    for (auto i = 0; i != ms3.extent(0); ++i) {
      cout << "Level " << i << ":\n";
      for (auto j = 0; j != ms3.extent(1); ++j) {
        for (auto k = 0; k != ms3.extent(2); ++k)
          cout << " " << ms3[i, j, k];  // read via multidimensional index
        cout << '\n';
      }
    }
    // Output: Level 0: 0 1, 2 3, 4 5, Level 1: 100 101, 102 103, 104 105
}
```

----------------


## Listing 24.30: Typically, you append at the end in vector.

Book listing [lst-0048-book.cpp](lst-0048-book.cpp):
```cpp
// https://godbolt.org/z/n6En49dsh
#include <vector>
#include <string>
struct President {
  std::string name_; int year_;
  President(std::string name,  int year)    // name is by Value
    : name_{std::move(name)}, year_{year}   // move saves an additional copy here
    { }
};
int main() {
    using namespace std;    // enable string literals
    vector<President> presidents{};
    presidents.emplace_back("Washington", 1789);
    presidents.emplace_back("Lincoln", 1861);
    presidents.emplace_back("Kennedy", 1961);
    // …
    vector<int> v{};
    v.reserve(100);
    for(int idx = 0; idx < 100; ++idx)
        v.push_back(idx);
    // …
}
```

Godbolt Listing [lst-0048-godb.cpp](lst-0048-godb.cpp), [https://godbolt.org/z/n6En49dsh](https://godbolt.org/z/n6En49dsh):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/n6En49dsh
#include <vector>
#include <string>
struct President {
  std::string name_; int year_;
  President(std::string name,  int year)    // name is by Value
    : name_{std::move(name)}, year_{year}   // move saves an additional copy here
    { }
};
int main() {
    using namespace std;    // enable string literals
    vector<President> presidents{};
    presidents.emplace_back("Washington", 1789);
    presidents.emplace_back("Lincoln", 1861);
    presidents.emplace_back("Kennedy", 1961);
    // …
    vector<int> v{};
    v.reserve(100);
    for(int idx = 0; idx < 100; ++idx)
        v.push_back(idx);
    // …
}
```

----------------


## GodboltId:6836bdczd

Book listing [lst-0051-book.cpp](lst-0051-book.cpp):
```cpp
// https://godbolt.org/z/6836bdczd 
std::vector v{ 1, 2, 3, 4, 5, 6 };
for(auto it=v.begin(); it!=v.end(); ++it) {
  it = v.erase(it);
}
// Here v is: { 2, 4, 6 }
```

Godbolt Listing [lst-0051-godb.cpp](lst-0051-godb.cpp), [https://godbolt.org/z/6836bdczd](https://godbolt.org/z/6836bdczd):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6836bdczd 
std::vector v{ 1, 2, 3, 4, 5, 6 };
for(auto it=v.begin(); it!=v.end(); ++it) {
  it = v.erase(it);
}
// Here v is: { 2, 4, 6 }
```

----------------


## Listing 24.31: erase with two parameters deletes an entire range.

Book listing [lst-0052-book.cpp](lst-0052-book.cpp):
```cpp
// https://godbolt.org/z/fe8bev9Y9 
std::vector v{ 1, 2, 3, 4, 5, 6 };
v.erase(v.begin()+2, v.begin()+5);  // v is now {1, 2, 6}
v.erase(v.begin(), v.end());        // deletes the rest
```

Godbolt Listing [lst-0052-godb.cpp](lst-0052-godb.cpp), [https://godbolt.org/z/fe8bev9Y9](https://godbolt.org/z/fe8bev9Y9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fe8bev9Y9 
std::vector v{ 1, 2, 3, 4, 5, 6 };
v.erase(v.begin()+2, v.begin()+5);  // v is now {1, 2, 6}
v.erase(v.begin(), v.end());        // deletes the rest
```

----------------


## Listing 24.32: Operations for the size and capacity of a "vector".

Book listing [lst-0054-book.cpp](lst-0054-book.cpp):
```cpp
// https://godbolt.org/z/YzGK1WazE
#include <vector>
#include <iostream>
#include <format> // C++20
using namespace std;
ostream& operator<<(ostream&os, const vector<int> &vec) {
  for(auto &elem : vec) { os << elem << ' '; } return os;
}
int main() {
  vector<int> data {};      // creates an empty vector
  data.reserve(3);          // Space for 3 elements
  cout << format("{}/{}\n", data.size(), data.capacity()); // 0/3
  data.push_back(111);      // add an element
  data.resize(3);           // Resize; here it appends new elements
  data.push_back(999);      // add another element
  cout << format("{}/{}\n", data.size(), data.capacity()); // 4/6
  cout << data << '\n';     // 111, 0, 0, 999
  data.push_back(333);      // add another element
  cout << data << '\n';     // 111, 0, 0, 999, 333
  data.reserve(1);          // nothing happens, because capacity() > 1
  data.resize(3);           // shrink; elements are removed
  cout << data << '\n';     // 111, 0, 0
  cout << format("{}/{}\n", data.size(), data.capacity()); // 3/6
  data.resize(6, 44);       // resize again, fill with 44s
  cout << data << '\n';     // 111, 0, 0, 44, 44, 44
}
```

Godbolt Listing [lst-0054-godb.cpp](lst-0054-godb.cpp), [https://godbolt.org/z/YzGK1WazE](https://godbolt.org/z/YzGK1WazE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/YzGK1WazE
#include <vector>
#include <iostream>
#include <format> // C++20
using namespace std;
ostream& operator<<(ostream&os, const vector<int> &vec) {
  for(auto &elem : vec) { os << elem << ' '; } return os;
}
int main() {
  vector<int> data {};      // creates an empty vector
  data.reserve(3);          // Space for 3 elements
  cout << format("{}/{}\n", data.size(), data.capacity()); // 0/3
  data.push_back(111);      // add an element
  data.resize(3);           // Resize; here it appends new elements
  data.push_back(999);      // add another element
  cout << format("{}/{}\n", data.size(), data.capacity()); // 4/6
  cout << data << '\n';     // 111, 0, 0, 999
  data.push_back(333);      // add another element
  cout << data << '\n';     // 111, 0, 0, 999, 333
  data.reserve(1);          // nothing happens, because capacity() > 1
  data.resize(3);           // shrink; elements are removed
  cout << data << '\n';     // 111, 0, 0
  cout << format("{}/{}\n", data.size(), data.capacity()); // 3/6
  data.resize(6, 44);       // resize again, fill with 44s
  cout << data << '\n';     // 111, 0, 0, 44, 44, 44
}
```

----------------


## GodboltId:Y1W8WfxYn

Book listing [lst-0055-book.cpp](lst-0055-book.cpp):
```cpp
// https://godbolt.org/z/Y1W8WfxYn 
#include <array>
void calculate(const std::array<int,4>& data) { /* ... */ }
void calculate(const std::array<int,5>& data) { /* ... */ }
```

Godbolt Listing [lst-0055-godb.cpp](lst-0055-godb.cpp), [https://godbolt.org/z/Y1W8WfxYn](https://godbolt.org/z/Y1W8WfxYn):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y1W8WfxYn 
#include <array>
void calculate(const std::array<int,4>& data) { /* ... */ }
void calculate(const std::array<int,5>& data) { /* ... */ }
```

----------------


## Listing 24.33: Different "arrays" as parameters require template programming.

Book listing [lst-0056-book.cpp](lst-0056-book.cpp):
```cpp
// https://godbolt.org/z/nqbKT9GEa 
#include <array>
#include <numeric> // accumulate
#include <iostream>
template<size_t SZ>
int sumSz(const std::array<int,SZ>& data) {
    int result = 0;
    for(auto i=0uz; i<SZ; ++i)        // uz is the suffix for size_t since C++23
        result += data[i];
    return result;
}
template<typename Elem, size_t SZ>
Elem sumElem(const std::array<Elem,SZ>& data) {
    Elem result {0};
    for(auto i=0uz; i<SZ; ++i)
        result += data[i];
    return result;
}
// C++20 concept input_iterator, otherwise typename
template<std::input_iterator It>
int sumIt(It begin, It end) {
    return std::accumulate(begin, end, 0);
}
// abbreviated function template with auto
int sumAuto(std::input_iterator auto begin, std::input_iterator auto end) {
    return std::accumulate(begin, end, 0);
}
int main() {
   using namespace std;
   array<int,4> a4 { 1, 2, 5, 8 };
   cout << sumSz<4>(a4) << '\n';                  // Output: 16
   array<int,5> a5 { 1, -5, 3, 7, 2 };
   cout << sumElem(a5) << '\n';                   // Output: 8
   array<int,6> a6 { 1,2,3, 4,5,6 };
   cout << sumIt(a6.begin(), a6.end()) << '\n';   // Output: 21
   array a7 { 1,2,3, 4,5,6, 7 };                  // array<int,7>
   cout << sumIt(a7.begin(), a7.end()) << '\n';   // Output: 28
   cout << sumAuto(a7.begin(), a7.end()) << '\n'; // Output: 28
}
```

Godbolt Listing [lst-0056-godb.cpp](lst-0056-godb.cpp), [https://godbolt.org/z/nqbKT9GEa](https://godbolt.org/z/nqbKT9GEa):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nqbKT9GEa 
#include <array>
#include <numeric> // accumulate
#include <iostream>
template<size_t SZ>
int sumSz(const std::array<int,SZ>& data) {
    int result = 0;
    for(auto i=0uz; i<SZ; ++i)        // uz is the suffix for size_t since C++23
        result += data[i];
    return result;
}
template<typename Elem, size_t SZ>
Elem sumElem(const std::array<Elem,SZ>& data) {
    Elem result {0};
    for(auto i=0uz; i<SZ; ++i)
        result += data[i];
    return result;
}
// C++20 concept input_iterator, otherwise typename
template<std::input_iterator It>
int sumIt(It begin, It end) {
    return std::accumulate(begin, end, 0);
}
// abbreviated function template with auto
int sumAuto(std::input_iterator auto begin, std::input_iterator auto end) {
    return std::accumulate(begin, end, 0);
}
int main() {
   using namespace std;
   array<int,4> a4 { 1, 2, 5, 8 };
   cout << sumSz<4>(a4) << '\n';                  // Output: 16
   array<int,5> a5 { 1, -5, 3, 7, 2 };
   cout << sumElem(a5) << '\n';                   // Output: 8
   array<int,6> a6 { 1,2,3, 4,5,6 };
   cout << sumIt(a6.begin(), a6.end()) << '\n';   // Output: 21
   array a7 { 1,2,3, 4,5,6, 7 };                  // array<int,7>
   cout << sumIt(a7.begin(), a7.end()) << '\n';   // Output: 28
   cout << sumAuto(a7.begin(), a7.end()) << '\n'; // Output: 28
}
```

----------------


## Listing 24.34: Template for the example listings of this section on "array".

Book listing [lst-0057-book.cpp](lst-0057-book.cpp):
```cpp
// https://godbolt.org/z/PrM4fGcWo 
#include <array>
#include <iostream>
using std::array; using std::cout;
template<typename Elem, size_t SZ>
std::ostream& operator<<(std::ostream&os, const array<Elem,SZ>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os;
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0057-godb.cpp](lst-0057-godb.cpp), [https://godbolt.org/z/PrM4fGcWo](https://godbolt.org/z/PrM4fGcWo):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/PrM4fGcWo 
#include <array>
#include <iostream>
using std::array; using std::cout;
template<typename Elem, size_t SZ>
std::ostream& operator<<(std::ostream&os, const array<Elem,SZ>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os;
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.36: "array" supports "get" from "tuple".

Book listing [lst-0058-book.cpp](lst-0058-book.cpp):
```cpp
// https://godbolt.org/z/KrbTrx1aG 
array<int,5> data{ 10, 11, 12, 13, 14};
cout << std::get<2>(data) << '\n'; // 12
```

Godbolt Listing [lst-0058-godb.cpp](lst-0058-godb.cpp), [https://godbolt.org/z/KrbTrx1aG](https://godbolt.org/z/KrbTrx1aG):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KrbTrx1aG 
array<int,5> data{ 10, 11, 12, 13, 14};
cout << std::get<2>(data) << '\n'; // 12
```

----------------


## Listing 24.37: An array supports structured binding.

Book listing [lst-0059-book.cpp](lst-0059-book.cpp):
```cpp
// https://godbolt.org/z/h6nrPYWfM 
#include <array>
#include <iostream>
int main() {
    std::array ports { 80, 443  };          // array<int>
    auto [ http, https ] = ports;           // declares variables and binds them
    std::cout << http << " " << https << "\n";
    auto const& [ rhttp, rhttps ] = ports;  // Reference and const are possible
    std::cout << rhttp << " " << rhttps << "\n";
}
```

Godbolt Listing [lst-0059-godb.cpp](lst-0059-godb.cpp), [https://godbolt.org/z/h6nrPYWfM](https://godbolt.org/z/h6nrPYWfM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h6nrPYWfM 
#include <array>
#include <iostream>
int main() {
    std::array ports { 80, 443  };          // array<int>
    auto [ http, https ] = ports;           // declares variables and binds them
    std::cout << http << " " << https << "\n";
    auto const& [ rhttp, rhttps ] = ports;  // Reference and const are possible
    std::cout << rhttp << " " << rhttps << "\n";
}
```

----------------


## Listing 24.38: Arrays can be compared lexicographically.

Book listing [lst-0060-book.cpp](lst-0060-book.cpp):
```cpp
// https://godbolt.org/z/j6EarszMT
// intentionally array& as arguments
void all(const array<int,4> &a, const array<int,4> &b) { 
     cout << "{"<<a<<"} compared with {"<<b<<"} is "
       << (a < b ? "<, " : "")
       << (a <= b ? "<=, " : "")
       << (a > b ? ">, " : "")
       << (a >= b ? ">=, " : "")
       << (a == b ? "==, " : "")
       << (a != b ? "!=, " : "")
       << '\n';
}
int main() {
    array a{10,10,10,10};
    array b{20, 5, 5, 5};
    array c{10,10,5,21};
    array d{10,10,10,10};
    cout << (a < b ? "smaller\n" : "not smaller\n"); // "smaller", because 10 < 20
    cout << (a < c ? "smaller\n" : "not smaller\n"); // "not smaller", because not 10 < 5
    cout << (a == d ? "equal\n" : "not equal\n");    // "equal", because all are 10
    for(auto &x : {a,b,c}) {
        for(auto &y : {a,b,c}) {
            all(x,y);
        }
    }
}
```

Godbolt Listing [lst-0060-godb.cpp](lst-0060-godb.cpp), [https://godbolt.org/z/j6EarszMT](https://godbolt.org/z/j6EarszMT):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/j6EarszMT
// intentionally array& as arguments
void all(const array<int,4> &a, const array<int,4> &b) { 
     cout << "{"<<a<<"} compared with {"<<b<<"} is "
       << (a < b ? "<, " : "")
       << (a <= b ? "<=, " : "")
       << (a > b ? ">, " : "")
       << (a >= b ? ">=, " : "")
       << (a == b ? "==, " : "")
       << (a != b ? "!=, " : "")
       << '\n';
}
int main() {
    array a{10,10,10,10};
    array b{20, 5, 5, 5};
    array c{10,10,5,21};
    array d{10,10,10,10};
    cout << (a < b ? "smaller\n" : "not smaller\n"); // "smaller", because 10 < 20
    cout << (a < c ? "smaller\n" : "not smaller\n"); // "not smaller", because not 10 < 5
    cout << (a == d ? "equal\n" : "not equal\n");    // "equal", because all are 10
    for(auto &x : {a,b,c}) {
        for(auto &y : {a,b,c}) {
            all(x,y);
        }
    }
}
```

----------------


## Listing 24.39: We remove pairs from the front and back and compare.

Book listing [lst-0061-book.cpp](lst-0061-book.cpp):
```cpp
// https://godbolt.org/z/Ee5anM9as
#include <deque>
#include <iostream>
#include <string>
#include <cctype>   // toupper
#include <iomanip>  // boolalpha
using namespace std;
bool isPalindrome(string_view word) {
    // check from both ends simultaneously
    deque<char> deq{};
    for(char ch : word) {
        deq.push_back(toupper(ch));    // uppercase
    }
    auto ok = true;
    while(deq.size()>1 && ok) {
       if(deq.front() != deq.back()) {
           ok = false;
       }
       deq.pop_front();                // Hello deque!
       deq.pop_back();
    }
    return ok;
}
int main() {
    cout << boolalpha; // Print 'true' and 'false' instead of '1' and '0'
    cout << isPalindrome("Abrakadabra") << '\n';   // false
    cout << isPalindrome("Kajak") << '\n';         // true
    cout << isPalindrome("EvilOlive") << '\n';     // true
    cout << isPalindrome("Aibohphobia") << '\n';   // true
    cout << isPalindrome("Madam") << '\n';         // true
    cout << isPalindrome("") << '\n';              // true
}
```

Godbolt Listing [lst-0061-godb.cpp](lst-0061-godb.cpp), [https://godbolt.org/z/Ee5anM9as](https://godbolt.org/z/Ee5anM9as):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Ee5anM9as
#include <deque>
#include <iostream>
#include <string>
#include <cctype>   // toupper
#include <iomanip>  // boolalpha
using namespace std;
bool isPalindrome(string_view word) {
    // check from both ends simultaneously
    deque<char> deq{};
    for(char ch : word) {
        deq.push_back(toupper(ch));    // uppercase
    }
    auto ok = true;
    while(deq.size()>1 && ok) {
       if(deq.front() != deq.back()) {
           ok = false;
       }
       deq.pop_front();                // Hello deque!
       deq.pop_back();
    }
    return ok;
}
int main() {
    cout << boolalpha; // Print 'true' and 'false' instead of '1' and '0'
    cout << isPalindrome("Abrakadabra") << '\n';   // false
    cout << isPalindrome("Kajak") << '\n';         // true
    cout << isPalindrome("EvilOlive") << '\n';     // true
    cout << isPalindrome("Aibohphobia") << '\n';   // true
    cout << isPalindrome("Madam") << '\n';         // true
    cout << isPalindrome("") << '\n';              // true
}
```

----------------


## Listing 24.40: What the "deque" can do!

Book listing [lst-0063-book.cpp](lst-0063-book.cpp):
```cpp
// https://godbolt.org/z/d3ExcWP4a
#include <deque>
#include <iostream>
#include <iterator>  // ostream_iterator
#include <algorithm> // copy
using namespace std;
ostream& operator<<=(ostream& os, int val) { return os<<val<<'\n'; }
int main() {
    deque d{ 4, 6 };            // Create deque<int> with elements
    // Insertion
    d.push_front(3);            // at the front: efficient
    d.insert(d.begin()+2, 5);   // in the middle: slow
    d.push_back(7);             // at the end: efficient
    // Access
    cout <<= d.front();         // from the front: efficient
    cout <<= d[3];              // in the middle: efficient
    cout <<= d.back();          // from the end: efficient
    // Size
    cout <<= d.size();          // read size
    d.resize(4);                // resize (or extend)
    // Iterators
    copy(d.begin(), d.end(), ostream_iterator<int>(cout, " "));
    cout << '\n';               // Output: 3 4 5 6
    // Remove
    d.pop_front();              // at the front: efficient
    d.erase(d.begin()+1);       // in the middle: slow
    d.pop_back();               // at the end: efficient
    d.clear();                  // clear
}
```

Godbolt Listing [lst-0063-godb.cpp](lst-0063-godb.cpp), [https://godbolt.org/z/d3ExcWP4a](https://godbolt.org/z/d3ExcWP4a):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/d3ExcWP4a
#include <deque>
#include <iostream>
#include <iterator>  // ostream_iterator
#include <algorithm> // copy
using namespace std;
ostream& operator<<=(ostream& os, int val) { return os<<val<<'\n'; }
int main() {
    deque d{ 4, 6 };            // Create deque<int> with elements
    // Insertion
    d.push_front(3);            // at the front: efficient
    d.insert(d.begin()+2, 5);   // in the middle: slow
    d.push_back(7);             // at the end: efficient
    // Access
    cout <<= d.front();         // from the front: efficient
    cout <<= d[3];              // in the middle: efficient
    cout <<= d.back();          // from the end: efficient
    // Size
    cout <<= d.size();          // read size
    d.resize(4);                // resize (or extend)
    // Iterators
    copy(d.begin(), d.end(), ostream_iterator<int>(cout, " "));
    cout << '\n';               // Output: 3 4 5 6
    // Remove
    d.pop_front();              // at the front: efficient
    d.erase(d.begin()+1);       // in the middle: slow
    d.pop_back();               // at the end: efficient
    d.clear();                  // clear
}
```

----------------


## Listing 24.41: "splice" is the specialty of "list" and efficiently connects two lists.

Book listing [lst-0064-book.cpp](lst-0064-book.cpp):
```cpp
// https://godbolt.org/z/so5Gebb4M 
#include <list>
#include <iostream>
using std::list; using std::cout; using std::ostream;

ostream& operator<<=(ostream&os, const list<int> &data)
    { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main() {
    list numa { 1, 3, 5, 7, 9 };
    list numb { 2, 4, 6, 8 };
    auto wh = numa.end();
    numa.splice(wh, numb); // transfer in O(1)
    cout <<= numa; // Output: 1 3 5 7 9 2 4 6 8
    cout <<= numb; // Output: (none)
    numa.sort();   // sort as a method, not from <algorithm>
    cout <<= numa; // Output: 1 2 3 4 5 6 7 8 9
}
```

Godbolt Listing [lst-0064-godb.cpp](lst-0064-godb.cpp), [https://godbolt.org/z/so5Gebb4M](https://godbolt.org/z/so5Gebb4M):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/so5Gebb4M 
#include <list>
#include <iostream>
using std::list; using std::cout; using std::ostream;

ostream& operator<<=(ostream&os, const list<int> &data)
    { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main() {
    list numa { 1, 3, 5, 7, 9 };
    list numb { 2, 4, 6, 8 };
    auto wh = numa.end();
    numa.splice(wh, numb); // transfer in O(1)
    cout <<= numa; // Output: 1 3 5 7 9 2 4 6 8
    cout <<= numb; // Output: (none)
    numa.sort();   // sort as a method, not from <algorithm>
    cout <<= numa; // Output: 1 2 3 4 5 6 7 8 9
}
```

----------------


## Listing 24.42: You can use "before_begin" as an argument for "insert_after".

Book listing [lst-0065-book.cpp](lst-0065-book.cpp):
```cpp
// https://godbolt.org/z/9b9fY4Mve 
#include <iostream>
#include <forward_list>
int main()     {
    std::forward_list mylist = {20, 30, 40, 50};
    mylist.insert_after(mylist.before_begin(), 11 );
    for (int& x: mylist) std::cout << ' ' << x; // Output: 11 20 30 40 50
    std::cout << '\n';
}
```

Godbolt Listing [lst-0065-godb.cpp](lst-0065-godb.cpp), [https://godbolt.org/z/9b9fY4Mve](https://godbolt.org/z/9b9fY4Mve):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9b9fY4Mve 
#include <iostream>
#include <forward_list>
int main()     {
    std::forward_list mylist = {20, 30, 40, 50};
    mylist.insert_after(mylist.before_begin(), 11 );
    for (int& x: mylist) std::cout << ' ' << x; // Output: 11 20 30 40 50
    std::cout << '\n';
}
```

----------------


## Listing 24.43: "erase_after" can delete a range of elements.

Book listing [lst-0066-book.cpp](lst-0066-book.cpp):
```cpp
// https://godbolt.org/z/andfoh18z
#include <forward_list>
#include <iostream>
#include <iterator> // next
using std::cout; using std::forward_list; using std::ostream;
ostream& operator<<=(ostream&os, const forward_list<int> &data)
    { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main()     {
    forward_list nums {40, 50, 60, 70};
    cout <<= nums;                         // Output: 40 50 60 70
    nums.insert_after(nums.before_begin(), {10, 20, 30});
    cout <<= nums;                         // Output: 10 20 30 40 50 60 70
    auto wh = std::next(nums.begin(), 2);  // two elements forward
    auto to = std::next(wh, 3);            // three elements after where
    nums.erase_after(wh, to);
    cout <<= nums;                         // Output: 10 20 30 60 70
}
```

Godbolt Listing [lst-0066-godb.cpp](lst-0066-godb.cpp), [https://godbolt.org/z/andfoh18z](https://godbolt.org/z/andfoh18z):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/andfoh18z
#include <forward_list>
#include <iostream>
#include <iterator> // next
using std::cout; using std::forward_list; using std::ostream;
ostream& operator<<=(ostream&os, const forward_list<int> &data)
    { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main()     {
    forward_list nums {40, 50, 60, 70};
    cout <<= nums;                         // Output: 40 50 60 70
    nums.insert_after(nums.before_begin(), {10, 20, 30});
    cout <<= nums;                         // Output: 10 20 30 40 50 60 70
    auto wh = std::next(nums.begin(), 2);  // two elements forward
    auto to = std::next(wh, 3);            // three elements after where
    nums.erase_after(wh, to);
    cout <<= nums;                         // Output: 10 20 30 60 70
}
```

----------------


## Listing 24.44: "splice_after" can very efficiently merge lists.

Book listing [lst-0067-book.cpp](lst-0067-book.cpp):
```cpp
// https://godbolt.org/z/Ka47W5Wjh 
#include <forward_list>
#include <iostream>
using std::cout; using std::forward_list; using std::ostream;

ostream& operator<<=(ostream&os, const forward_list<int> &data)
  { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main() {
  {
    forward_list fw1 {10, 20, 30, 40};
    forward_list fw2 {5, 6, 7, 8};
    fw1.splice_after(fw1.begin(), fw2);  // transfers everything
    cout <<= fw1;                        // Output: 10 5 6 7 8 20 30 40
    cout <<= fw2;                        // Output:
  }
  {
    forward_list fw1 {10, 20, 30, 40};
    forward_list fw2 {5, 6, 7, 8};
    // one element left:
    fw1.splice_after(fw1.begin(), fw2,fw2.begin(),fw2.end());
    cout <<= fw1;                        // Output: 10 6 7 8 20 30 40
    cout <<= fw2;                        // Output: 5
  }
}
```

Godbolt Listing [lst-0067-godb.cpp](lst-0067-godb.cpp), [https://godbolt.org/z/Ka47W5Wjh](https://godbolt.org/z/Ka47W5Wjh):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Ka47W5Wjh 
#include <forward_list>
#include <iostream>
using std::cout; using std::forward_list; using std::ostream;

ostream& operator<<=(ostream&os, const forward_list<int> &data)
  { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main() {
  {
    forward_list fw1 {10, 20, 30, 40};
    forward_list fw2 {5, 6, 7, 8};
    fw1.splice_after(fw1.begin(), fw2);  // transfers everything
    cout <<= fw1;                        // Output: 10 5 6 7 8 20 30 40
    cout <<= fw2;                        // Output:
  }
  {
    forward_list fw1 {10, 20, 30, 40};
    forward_list fw2 {5, 6, 7, 8};
    // one element left:
    fw1.splice_after(fw1.begin(), fw2,fw2.begin(),fw2.end());
    cout <<= fw1;                        // Output: 10 6 7 8 20 30 40
    cout <<= fw2;                        // Output: 5
  }
}
```

----------------


## Listing 24.45: This is the template for the example listings in this section on "set".

Book listing [lst-0068-book.cpp](lst-0068-book.cpp):
```cpp
// https://godbolt.org/z/7b8GaE9xT
#include <set>
#include <iostream>
using std::set; using std::cout;
template<typename Elem, typename Comp>
std::ostream& operator<<=(std::ostream&os, const set<Elem,Comp>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0068-godb.cpp](lst-0068-godb.cpp), [https://godbolt.org/z/7b8GaE9xT](https://godbolt.org/z/7b8GaE9xT):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/7b8GaE9xT
#include <set>
#include <iostream>
using std::set; using std::cout;
template<typename Elem, typename Comp>
std::ostream& operator<<=(std::ostream&os, const set<Elem,Comp>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.46: If you define the comparison operation incorrectly, then "set" will no longer work.

Book listing [lst-0070-book.cpp](lst-0070-book.cpp):
```cpp
// https://godbolt.org/z/jfdh3ddsz
using std::cout; using std::ostream; using std::set;
auto comp = [](auto e, auto f) { return e <= f; }; // (ERR) defined wrong!
std::set<int,decltype(comp)> data(comp);
data.insert({ 9,5,7,2,3,6,8 });
cout <<= data;                    // Output with luck: 2 3 5 6 7 8
auto it = data.find(7);
if(it != data.end()) {
    cout << "got it: " << *it << '\n';
} else {
    cout << "it's gone!" << '\n'; // you will probably end up here
}
```

Godbolt Listing [lst-0070-godb.cpp](lst-0070-godb.cpp), [https://godbolt.org/z/jfdh3ddsz](https://godbolt.org/z/jfdh3ddsz):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jfdh3ddsz
using std::cout; using std::ostream; using std::set;
auto comp = [](auto e, auto f) { return e <= f; }; // (ERR) defined wrong!
std::set<int,decltype(comp)> data(comp);
data.insert({ 9,5,7,2,3,6,8 });
cout <<= data;                    // Output with luck: 2 3 5 6 7 8
auto it = data.find(7);
if(it != data.end()) {
    cout << "got it: " << *it << '\n';
} else {
    cout << "it's gone!" << '\n'; // you will probably end up here
}
```

----------------


## Listing 24.47: The comparison operation can group elements.

Book listing [lst-0071-book.cpp](lst-0071-book.cpp):
```cpp
// https://godbolt.org/z/xdPc51q5s
auto comp = [](auto e, auto f) {return e/10<f/10;}; // grouping is okay
std::set<int,decltype(comp)> data(comp);
data.insert({ 14,23,56,71 });
cout <<= data;                       // Output: 14 23 56 71
auto it = data.find(29);             // 29 now also finds 23
if(it != data.end()) {
  cout << "got it: " << *it << '\n'; // Output: got it: 23
}
data.insert(79);                     // nothing happens, 71 is already in
cout <<= data;                       // Output: 14 23 56 71
```

Godbolt Listing [lst-0071-godb.cpp](lst-0071-godb.cpp), [https://godbolt.org/z/xdPc51q5s](https://godbolt.org/z/xdPc51q5s):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xdPc51q5s
auto comp = [](auto e, auto f) {return e/10<f/10;}; // grouping is okay
std::set<int,decltype(comp)> data(comp);
data.insert({ 14,23,56,71 });
cout <<= data;                       // Output: 14 23 56 71
auto it = data.find(29);             // 29 now also finds 23
if(it != data.end()) {
  cout << "got it: " << *it << '\n'; // Output: got it: 23
}
data.insert(79);                     // nothing happens, 71 is already in
cout <<= data;                       // Output: 14 23 56 71
```

----------------


## Listing 24.48: A custom spaceship operator for "set" compatibility.

Book listing [lst-0072-book.cpp](lst-0072-book.cpp):
```cpp
// https://godbolt.org/z/W9sK4jnzK
#include <iostream>
#include <set>
#include <string>
struct Dwarf {
  std::string name;
  int height;
  auto operator<=>(const Dwarf& other) const {
    return height <=> other.height;
  }
};
int main() {
  std::set<Dwarf> dwarves {
    {"Thorin", 140}, {"Balin", 136}, {"Kili", 138},
    {"Dwalin", 139}, {"Oin", 135},   {"Gloin", 137},
  };
  for(auto &d: dwarves) {
    std::cout << d.name << ' ';
  } // Output: Oin Balin Kili Gloin Dwalin Thorin
}
```

Godbolt Listing [lst-0072-godb.cpp](lst-0072-godb.cpp), [https://godbolt.org/z/W9sK4jnzK](https://godbolt.org/z/W9sK4jnzK):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/W9sK4jnzK
#include <iostream>
#include <set>
#include <string>
struct Dwarf {
  std::string name;
  int height;
  auto operator<=>(const Dwarf& other) const {
    return height <=> other.height;
  }
};
int main() {
  std::set<Dwarf> dwarves {
    {"Thorin", 140}, {"Balin", 136}, {"Kili", 138},
    {"Dwalin", 139}, {"Oin", 135},   {"Gloin", 137},
  };
  for(auto &d: dwarves) {
    std::cout << d.name << ' ';
  } // Output: Oin Balin Kili Gloin Dwalin Thorin
}
```

----------------


## Listing 24.49: There are various ways to specify a comparator.

Book listing [lst-0074-book.cpp](lst-0074-book.cpp):
```cpp
// https://godbolt.org/z/c8a9qPoPP
#include <set>
#include <functional> // function
using std::set; using std::function; using std::initializer_list;
bool fcompTens(int a, int b) { return a%10 < b%10; }
struct Fives {
    bool operator()(int a, int b) const { return a%5 < b% 5; }
};

int main() {
    // Functor
    set<int, Fives> ff1;
    ff1.insert(5);
    set ff2({5}, Fives{}); 
    set ff3(initializer_list<int>({}), Fives{});
    // Lambda
    set<int,function<bool(int,int)>> ll1([](auto a,auto b){return a%3<b%3;});
    ll1.insert(3);
    auto lcomp = [](int a, int b) { return a%3 < b%3; };
    set<int, decltype(lcomp)> ll2(lcomp);
    ll2.insert(3);
    set ll3({3}, lcomp); 
    // Function pointer
    set<int, bool(*)(int,int)> zz1(&fcompTens);        // C-style
    zz1.insert(10);
    set<int, function<bool(int,int)>> zz2(&fcompTens); // C++ style
    zz2.insert(10);
    set<int, decltype(&fcompTens)> zz3(&fcompTens);    // C++ style
    zz3.insert(10);
}
```

Godbolt Listing [lst-0074-godb.cpp](lst-0074-godb.cpp), [https://godbolt.org/z/c8a9qPoPP](https://godbolt.org/z/c8a9qPoPP):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/c8a9qPoPP
#include <set>
#include <functional> // function
using std::set; using std::function; using std::initializer_list;
bool fcompTens(int a, int b) { return a%10 < b%10; }
struct Fives {
    bool operator()(int a, int b) const { return a%5 < b% 5; }
};

int main() {
    // Functor
    set<int, Fives> ff1;
    ff1.insert(5);
    set ff2({5}, Fives{}); 
    set ff3(initializer_list<int>({}), Fives{});
    // Lambda
    set<int,function<bool(int,int)>> ll1([](auto a,auto b){return a%3<b%3;});
    ll1.insert(3);
    auto lcomp = [](int a, int b) { return a%3 < b%3; };
    set<int, decltype(lcomp)> ll2(lcomp);
    ll2.insert(3);
    set ll3({3}, lcomp); 
    // Function pointer
    set<int, bool(*)(int,int)> zz1(&fcompTens);        // C-style
    zz1.insert(10);
    set<int, function<bool(int,int)>> zz2(&fcompTens); // C++ style
    zz2.insert(10);
    set<int, decltype(&fcompTens)> zz3(&fcompTens);    // C++ style
    zz3.insert(10);
}
```

----------------


## Listing 24.50: There are again several ways to specify elements when constructing.

Book listing [lst-0075-book.cpp](lst-0075-book.cpp):
```cpp
// https://godbolt.org/z/oc9xxev95
// without arguments
set<int> empty{};
cout <<= empty;          // Output:
// initializer list
set list{ 1,1,2,2,3,3,4,4,5,5 };  // set does not take duplicates
cout <<= list;           // Output: 1 2 3 4 5
// copy
set copy(list);
cout <<= copy;           // Output: 1 2 3 4 5
// pair of iterators
set from_to( std::next(list.begin()), std::prev(list.end()));
cout <<= from_to;        // Output: 2 3 4
// Range
set even(from_range, list | vs::filter([](int i){ return i%2; }));
cout <<= from_to;        // Output: 2 4
```

Godbolt Listing [lst-0075-godb.cpp](lst-0075-godb.cpp), [https://godbolt.org/z/oc9xxev95](https://godbolt.org/z/oc9xxev95):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/oc9xxev95
// without arguments
set<int> empty{};
cout <<= empty;          // Output:
// initializer list
set list{ 1,1,2,2,3,3,4,4,5,5 };  // set does not take duplicates
cout <<= list;           // Output: 1 2 3 4 5
// copy
set copy(list);
cout <<= copy;           // Output: 1 2 3 4 5
// pair of iterators
set from_to( std::next(list.begin()), std::prev(list.end()));
cout <<= from_to;        // Output: 2 3 4
// Range
set even(from_range, list | vs::filter([](int i){ return i%2; }));
cout <<= from_to;        // Output: 2 4
```

----------------


## GodboltId:1dTPEd3nP

Book listing [lst-0076-book.cpp](lst-0076-book.cpp):
```cpp
// https://godbolt.org/z/1dTPEd3nP 
set source{1,2,3,4,5};
cout <<= source;         // Output: 1 2 3 4 5
set target( std::move(source) ); // move instead of copy
cout <<= source;         // Output:
cout <<= target;         // Output: 1 2 3 4 5
```

Godbolt Listing [lst-0076-godb.cpp](lst-0076-godb.cpp), [https://godbolt.org/z/1dTPEd3nP](https://godbolt.org/z/1dTPEd3nP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1dTPEd3nP 
set source{1,2,3,4,5};
cout <<= source;         // Output: 1 2 3 4 5
set target( std::move(source) ); // move instead of copy
cout <<= source;         // Output:
cout <<= target;         // Output: 1 2 3 4 5
```

----------------


## GodboltId:8eGvWY45o

Book listing [lst-0077-book.cpp](lst-0077-book.cpp):
```cpp
// https://godbolt.org/z/8eGvWY45o 
set source{1,2,3,4,5};
set<int> target{};
set<int> target2{};
cout <<= source;         // Output: 1 2 3 4 5
cout <<= target;         // Output:
cout <<= target2;        // Output:
target = source;         // copy afterward
cout <<= source;         // Output: 1 2 3 4 5
cout <<= target;         // Output: 1 2 3 4 5
target2 = std::move(source); // move
cout <<= source;         // Output:
cout <<= target2;        // Output: 1 2 3 4 5
```

Godbolt Listing [lst-0077-godb.cpp](lst-0077-godb.cpp), [https://godbolt.org/z/8eGvWY45o](https://godbolt.org/z/8eGvWY45o):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8eGvWY45o 
set source{1,2,3,4,5};
set<int> target{};
set<int> target2{};
cout <<= source;         // Output: 1 2 3 4 5
cout <<= target;         // Output:
cout <<= target2;        // Output:
target = source;         // copy afterward
cout <<= source;         // Output: 1 2 3 4 5
cout <<= target;         // Output: 1 2 3 4 5
target2 = std::move(source); // move
cout <<= source;         // Output:
cout <<= target2;        // Output: 1 2 3 4 5
```

----------------


## Listing 24.51: Instead of »assign« you can use the copy-and-swap idiom.

Book listing [lst-0078-book.cpp](lst-0078-book.cpp):
```cpp
// https://godbolt.org/z/ozMY86ns8
#include <vector>
// …
set data{1,2,3,4,5};
std::vector source{10, 20, 30, 40, 50};

// There is no set::assign:
data.assign(source.begin(), source.end());   // (ERR) no set::assign
// So simulate it using a temporary set:
set temp(source.begin(), source.end());      // copy from source …
data.swap(temp);                             // … efficiently swap contents
cout <<= data;                  // Output: 10 20 30 40 50
// … or by clearing first and then inserting:
data.clear();                                // clear …
data.insert(source.begin(), source.end());   // … and insert
cout <<= data;                  // Output: 10 20 30 40 50
```

Godbolt Listing [lst-0078-godb.cpp](lst-0078-godb.cpp), [https://godbolt.org/z/ozMY86ns8](https://godbolt.org/z/ozMY86ns8):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ozMY86ns8
#include <vector>
// …
set data{1,2,3,4,5};
std::vector source{10, 20, 30, 40, 50};

// There is no set::assign:
data.assign(source.begin(), source.end());   // (ERR) no set::assign
// So simulate it using a temporary set:
set temp(source.begin(), source.end());      // copy from source …
data.swap(temp);                             // … efficiently swap contents
cout <<= data;                  // Output: 10 20 30 40 50
// … or by clearing first and then inserting:
data.clear();                                // clear …
data.insert(source.begin(), source.end());   // … and insert
cout <<= data;                  // Output: 10 20 30 40 50
```

----------------


## Listing 24.52: To insert a single element, use "insert" or "emplace".

Book listing [lst-0079-book.cpp](lst-0079-book.cpp):
```cpp
// https://godbolt.org/z/8nj7vfeba
// …
template<typename IT> ostream& operator<<(ostream& os,const pair<IT,bool> wh)
  { return os << (wh.second ? "yes" : "no"); }
struct Point {
    double x_, y_;
    Point(double x, double y) : x_{x}, y_{y} {}
    auto operator<=>(const Point&) const = default;
    friend ostream& operator<<(ostream &os, const Point &a) {
        return os << "(" << a.x_ << ',' << a.y_<< ")";
    }
};
int main() {
    set data{ 10, 20, 30, 40, 50, 60, 70 };
    auto wh = data.insert(35);         // inserts between 30 and 40
    cout << "new? " << wh << '\n';     // Output: new? yes
    wh = data.insert(40);              // already exists, so does not insert
    cout << "new? " << wh << '\n';     // Output: new? no
    set<Point> points{};
    points.insert( Point{3.50,7.25} ); // temporary value
    points.emplace(1.25, 2.00);        // constructor arguments
    cout <<= points;                   // Output: (1.25,2) (3.5,7.25)
}
```

Godbolt Listing [lst-0079-godb.cpp](lst-0079-godb.cpp), [https://godbolt.org/z/8nj7vfeba](https://godbolt.org/z/8nj7vfeba):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8nj7vfeba
// …
template<typename IT> ostream& operator<<(ostream& os,const pair<IT,bool> wh)
  { return os << (wh.second ? "yes" : "no"); }
struct Point {
    double x_, y_;
    Point(double x, double y) : x_{x}, y_{y} {}
    auto operator<=>(const Point&) const = default;
    friend ostream& operator<<(ostream &os, const Point &a) {
        return os << "(" << a.x_ << ',' << a.y_<< ")";
    }
};
int main() {
    set data{ 10, 20, 30, 40, 50, 60, 70 };
    auto wh = data.insert(35);         // inserts between 30 and 40
    cout << "new? " << wh << '\n';     // Output: new? yes
    wh = data.insert(40);              // already exists, so does not insert
    cout << "new? " << wh << '\n';     // Output: new? no
    set<Point> points{};
    points.insert( Point{3.50,7.25} ); // temporary value
    points.emplace(1.25, 2.00);        // constructor arguments
    cout <<= points;                   // Output: (1.25,2) (3.5,7.25)
}
```

----------------


## Listing 24.53: You can reuse the return value when inserting sorted ranges.

Book listing [lst-0080-book.cpp](lst-0080-book.cpp):
```cpp
// https://godbolt.org/z/qzaj79bor 
set data{ 10, 20, 30, 40, 50, 60, 70 };
set<int> target;
auto hint = target.begin();
for(auto &e : data) {
    hint =                      // Use insertion position in the next round
        target.insert(hint, e); // Hint helps because data is sorted
}
```

Godbolt Listing [lst-0080-godb.cpp](lst-0080-godb.cpp), [https://godbolt.org/z/qzaj79bor](https://godbolt.org/z/qzaj79bor):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qzaj79bor 
set data{ 10, 20, 30, 40, 50, 60, 70 };
set<int> target;
auto hint = target.begin();
for(auto &e : data) {
    hint =                      // Use insertion position in the next round
        target.insert(hint, e); // Hint helps because data is sorted
}
```

----------------


## Listing 24.54: Use the same insert for sequential and associative containers.

Book listing [lst-0081-book.cpp](lst-0081-book.cpp):
```cpp
// https://godbolt.org/z/h1G8v86jv
#include <set>
#include <vector>
#include <iostream>
using std::cout; using std::ostream; using std::set; using std::vector;

template<typename Container>
void insFive(Container& cont, int a, int b, int c, int d, int e) {
    auto it = cont.end();
    for(int x : { a, b, c, d, e }) {
        it = cont.insert(it, x); // works with vector, set etc.
    }
}
int main() {
    vector<int> dataVec{ };
    insFive(dataVec, 9, 2, 2, 0, 4 );
    for(auto e : dataVec) cout <<e<<' ';
    cout << '\n';                // Output: 4 0 2 2 9
    set<int> dataSet{ };
    insFive(dataSet, 9, 4, 2, 2, 0);
    for(auto e : dataSet) cout <<e<<' ';
    cout << '\n';                // Output: 0 2 4 9
}
```

Godbolt Listing [lst-0081-godb.cpp](lst-0081-godb.cpp), [https://godbolt.org/z/h1G8v86jv](https://godbolt.org/z/h1G8v86jv):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/h1G8v86jv
#include <set>
#include <vector>
#include <iostream>
using std::cout; using std::ostream; using std::set; using std::vector;

template<typename Container>
void insFive(Container& cont, int a, int b, int c, int d, int e) {
    auto it = cont.end();
    for(int x : { a, b, c, d, e }) {
        it = cont.insert(it, x); // works with vector, set etc.
    }
}
int main() {
    vector<int> dataVec{ };
    insFive(dataVec, 9, 2, 2, 0, 4 );
    for(auto e : dataVec) cout <<e<<' ';
    cout << '\n';                // Output: 4 0 2 2 9
    set<int> dataSet{ };
    insFive(dataSet, 9, 4, 2, 2, 0);
    for(auto e : dataSet) cout <<e<<' ';
    cout << '\n';                // Output: 0 2 4 9
}
```

----------------


## Listing 24.55: You can also insert multiple elements.

Book listing [lst-0082-book.cpp](lst-0082-book.cpp):
```cpp
// https://godbolt.org/z/rTvovhr5M 
#include <vector>
set data{ 10, 20, 30, };
data.insert( { 40, 50, 60, 70 }); // Initializer list
std::vector new_vec{ 5, 25, 35, 15, 25, 75, 95 };
data.insert( new_vec.cbegin()+1, new_vec.cend()-1 ); // area
cout <<= data; // Output: 10 15 20 25 30 35 40 50 60 70 75
```

Godbolt Listing [lst-0082-godb.cpp](lst-0082-godb.cpp), [https://godbolt.org/z/rTvovhr5M](https://godbolt.org/z/rTvovhr5M):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rTvovhr5M 
#include <vector>
set data{ 10, 20, 30, };
data.insert( { 40, 50, 60, 70 }); // Initializer list
std::vector new_vec{ 5, 25, 35, 15, 25, 75, 95 };
data.insert( new_vec.cbegin()+1, new_vec.cend()-1 ); // area
cout <<= data; // Output: 10 15 20 25 30 35 40 50 60 70 75
```

----------------


## Listing 24.56: These are the "set" search functions.

Book listing [lst-0083-book.cpp](lst-0083-book.cpp):
```cpp
// https://godbolt.org/z/MW74f97f7
#include <set>
#include <iostream>
using std::cout; using std::ostream; using std::set;
void search(const set<int>&data, int what, ostream&os) {
    os << what << "? ";
    // contains
    auto inside = data.contains(what);  // C++20
    os << "inside:" << (inside ? "yes." : "no."); // check contains
    auto where = data.find(what);
    if(where != data.end()) {
        os << " found:" << *where << " ";
    } else {
        os << " not found. ";
    }
    auto lo = data.lower_bound(what);
    if(lo != data.end()) {
        os << "lo:" << *lo;
    } else {
        os << "lo:-";
    }
    auto up = data.upper_bound(what);
    if(up != data.end()) {
        os << " up:" << *up;
    } else {
        os << " up:-";
    }
    // [lo,up] is now the same as what equal_range would have returned
    os << " Range:{";
    for( ; lo != up; ++ lo) {
      os << *lo << ' ';
    }
    os << "}";
    // count
    os << " C:" << data.count(what) // count hits
       << "\n";
}
int main() {
    set data{ 10, 20, 30, 40, 50, 60 };
    search(data, 20, cout); // 20? in:yes. found:20 lo:20 up:30 range:{20 } C:1
    search(data, 25, cout); // 25? in:no. lo:30 up:30 range:{} C:0
    search(data, 10, cout); // 10? in:yes. found:10 lo:10 up:20 range:{10 } C:1
    search(data, 60, cout); // 60? in:yes. found:60 lo:60 up:- range:{60 } C:1
    search(data, 5, cout);  // 5? in: no. lo:10 up:10 Range:{} C:0
    search(data, 99, cout); // 99? in: no. lo:- up:- Range:{} C:0
}
```

Godbolt Listing [lst-0083-godb.cpp](lst-0083-godb.cpp), [https://godbolt.org/z/MW74f97f7](https://godbolt.org/z/MW74f97f7):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/MW74f97f7
#include <set>
#include <iostream>
using std::cout; using std::ostream; using std::set;
void search(const set<int>&data, int what, ostream&os) {
    os << what << "? ";
    // contains
    auto inside = data.contains(what);  // C++20
    os << "inside:" << (inside ? "yes." : "no."); // check contains
    auto where = data.find(what);
    if(where != data.end()) {
        os << " found:" << *where << " ";
    } else {
        os << " not found. ";
    }
    auto lo = data.lower_bound(what);
    if(lo != data.end()) {
        os << "lo:" << *lo;
    } else {
        os << "lo:-";
    }
    auto up = data.upper_bound(what);
    if(up != data.end()) {
        os << " up:" << *up;
    } else {
        os << " up:-";
    }
    // [lo,up] is now the same as what equal_range would have returned
    os << " Range:{";
    for( ; lo != up; ++ lo) {
      os << *lo << ' ';
    }
    os << "}";
    // count
    os << " C:" << data.count(what) // count hits
       << "\n";
}
int main() {
    set data{ 10, 20, 30, 40, 50, 60 };
    search(data, 20, cout); // 20? in:yes. found:20 lo:20 up:30 range:{20 } C:1
    search(data, 25, cout); // 25? in:no. lo:30 up:30 range:{} C:0
    search(data, 10, cout); // 10? in:yes. found:10 lo:10 up:20 range:{10 } C:1
    search(data, 60, cout); // 60? in:yes. found:60 lo:60 up:- range:{60 } C:1
    search(data, 5, cout);  // 5? in: no. lo:10 up:10 Range:{} C:0
    search(data, 99, cout); // 99? in: no. lo:- up:- Range:{} C:0
}
```

----------------


## Listing 24.57: You can search with nonidentical keys if they are equivalent.

Book listing [lst-0084-book.cpp](lst-0084-book.cpp):
```cpp
// https://godbolt.org/z/zEsxd8e7G
#include <string>
#include <set>
#include <iostream>
#include <tuple> // tuple, tie
using std::string; using std::set; using std::cout; using std::tie;
struct Hobbit {
  string firstName;
  string lastName;
  Hobbit(const string v, const string n) : firstName{v}, lastName{n} {}
};
struct CompLastName {
  bool operator()(const Hobbit& x, const Hobbit& y) const { // normal <
    return tie(x.lastName, x.firstName) < tie(y.lastName, y.firstName);
  }
  using is_transparent = std::true_type; // allowed for find
  bool operator()(const Hobbit& x, const string& y) const { // for find etc.
    return x.lastName < y;
  }
  bool operator()(const string& x, const Hobbit& y) const { // for find etc.
    return x < y.lastName;
  }
};
int main() {
    using namespace std::literals; // allow "…"s
    set<Hobbit,CompLastName> hobbits;
    hobbits.emplace( "Frodo", "Baggins" );
    hobbits.emplace( "Sam", "Gamgee" );
    auto f1 = hobbits.find( Hobbit{"Frodo", "Baggins"} ); // whole key
    if(f1 != hobbits.end()) {
        cout << "found: " << f1->firstName << '\n';       // Frodo
    }
    auto f2 = hobbits.find( "Gamgee"s );                  // equivalent key
    if(f2 != hobbits.end()) {
        cout << "found: " << f2->firstName << '\n';       // Sam
    }
}
```

Godbolt Listing [lst-0084-godb.cpp](lst-0084-godb.cpp), [https://godbolt.org/z/zEsxd8e7G](https://godbolt.org/z/zEsxd8e7G):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zEsxd8e7G
#include <string>
#include <set>
#include <iostream>
#include <tuple> // tuple, tie
using std::string; using std::set; using std::cout; using std::tie;
struct Hobbit {
  string firstName;
  string lastName;
  Hobbit(const string v, const string n) : firstName{v}, lastName{n} {}
};
struct CompLastName {
  bool operator()(const Hobbit& x, const Hobbit& y) const { // normal <
    return tie(x.lastName, x.firstName) < tie(y.lastName, y.firstName);
  }
  using is_transparent = std::true_type; // allowed for find
  bool operator()(const Hobbit& x, const string& y) const { // for find etc.
    return x.lastName < y;
  }
  bool operator()(const string& x, const Hobbit& y) const { // for find etc.
    return x < y.lastName;
  }
};
int main() {
    using namespace std::literals; // allow "…"s
    set<Hobbit,CompLastName> hobbits;
    hobbits.emplace( "Frodo", "Baggins" );
    hobbits.emplace( "Sam", "Gamgee" );
    auto f1 = hobbits.find( Hobbit{"Frodo", "Baggins"} ); // whole key
    if(f1 != hobbits.end()) {
        cout << "found: " << f1->firstName << '\n';       // Frodo
    }
    auto f2 = hobbits.find( "Gamgee"s );                  // equivalent key
    if(f2 != hobbits.end()) {
        cout << "found: " << f2->firstName << '\n';       // Sam
    }
}
```

----------------


## Listing 24.58: "erase" deletes one or more elements.

Book listing [lst-0086-book.cpp](lst-0086-book.cpp):
```cpp
// https://godbolt.org/z/dh8Yqx6c4
set data{ 10, 20, 30, 40, 50, 60, 70 };
auto lo = data.lower_bound(35);
auto up = data.upper_bound(55);
data.erase(lo, up);       // deletes all numbers between 35 and 55
cout <<= data;            // Output: 10 20 30 60 70
lo = data.lower_bound(20);
up = data.upper_bound(60);
data.erase(lo, up);      // deletes including 60, because up points to 70
cout <<= data;           // Output: 10 70
auto n = data.erase(69); // deletes nothing
cout << "Number of removed elements: "<< n << '\n'; // Output: Number … 0
n = data.erase(70);      // deletes one element
cout << "Number of removed elements: "<< n << '\n';  // Output: Number … 1
cout <<= data;           // Output: 10
```

Godbolt Listing [lst-0086-godb.cpp](lst-0086-godb.cpp), [https://godbolt.org/z/dh8Yqx6c4](https://godbolt.org/z/dh8Yqx6c4):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/dh8Yqx6c4
set data{ 10, 20, 30, 40, 50, 60, 70 };
auto lo = data.lower_bound(35);
auto up = data.upper_bound(55);
data.erase(lo, up);       // deletes all numbers between 35 and 55
cout <<= data;            // Output: 10 20 30 60 70
lo = data.lower_bound(20);
up = data.upper_bound(60);
data.erase(lo, up);      // deletes including 60, because up points to 70
cout <<= data;           // Output: 10 70
auto n = data.erase(69); // deletes nothing
cout << "Number of removed elements: "<< n << '\n'; // Output: Number … 0
n = data.erase(70);      // deletes one element
cout << "Number of removed elements: "<< n << '\n';  // Output: Number … 1
cout <<= data;           // Output: 10
```

----------------


## Listing 24.59: Using [] may create an entry as a side effect.

Book listing [lst-0087-book.cpp](lst-0087-book.cpp):
```cpp
// https://godbolt.org/z/zWa58dYYq 
#include <map>
#include <iostream>
using std::map; using std::cout;
int main() {
    map<int,char> alpha;
    cout << alpha.size() << '\n';        // 0 naturally
    if( alpha[5] == '3' ) { /* ... */ }
    cout << alpha.size() << '\n';        // now 1
    char x = alpha[99];                  // works
    cout << alpha.size() << '\n';        // and now 2
}
```

Godbolt Listing [lst-0087-godb.cpp](lst-0087-godb.cpp), [https://godbolt.org/z/zWa58dYYq](https://godbolt.org/z/zWa58dYYq):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zWa58dYYq 
#include <map>
#include <iostream>
using std::map; using std::cout;
int main() {
    map<int,char> alpha;
    cout << alpha.size() << '\n';        // 0 naturally
    if( alpha[5] == '3' ) { /* ... */ }
    cout << alpha.size() << '\n';        // now 1
    char x = alpha[99];                  // works
    cout << alpha.size() << '\n';        // and now 2
}
```

----------------


## Listing 24.60: This is the template for the example listings in this section on "map".

Book listing [lst-0088-book.cpp](lst-0088-book.cpp):
```cpp
// https://godbolt.org/z/K834js588
#include <map>        // the main thing
#include <iostream>   // for output
#include <string>     // often used for key or target
using std::map; using std::cout; using std::string;
template<typename Key, typename Trg, typename Comp>
std::ostream& operator<<=(std::ostream&os, const map<Key,Trg,Comp>&data) {
    for(auto &e : data) os << e.first << ":" << e.second << ' ';
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0088-godb.cpp](lst-0088-godb.cpp), [https://godbolt.org/z/K834js588](https://godbolt.org/z/K834js588):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/K834js588
#include <map>        // the main thing
#include <iostream>   // for output
#include <string>     // often used for key or target
using std::map; using std::cout; using std::string;
template<typename Key, typename Trg, typename Comp>
std::ostream& operator<<=(std::ostream&os, const map<Key,Trg,Comp>&data) {
    for(auto &e : data) os << e.first << ":" << e.second << ' ';
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.61: You can also provide a custom comparison function for a "map".

Book listing [lst-0089-book.cpp](lst-0089-book.cpp):
```cpp
// https://godbolt.org/z/W6Mfnxj1W
#include <cstdio> // toupper, tolower
// …
auto comp = [](char a, char b) { return toupper(a) < toupper(b); };
map<char,int,decltype(comp)> lets(comp); // as template parameter and argument
lets['a'] = 1;
lets['B'] = 2;
lets['c'] = 3;
lets['A'] = 4; // overwrites position 'a'
cout <<= lets; // Output: a:4 B:2 c:3
struct Comp {  // Functor
    bool operator()(char a, char b) const { return toupper(a) < toupper(b); }
};
map<char,int,Comp> lets2; // here the template parameter is sufficient
```

Godbolt Listing [lst-0089-godb.cpp](lst-0089-godb.cpp), [https://godbolt.org/z/W6Mfnxj1W](https://godbolt.org/z/W6Mfnxj1W):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/W6Mfnxj1W
#include <cstdio> // toupper, tolower
// …
auto comp = [](char a, char b) { return toupper(a) < toupper(b); };
map<char,int,decltype(comp)> lets(comp); // as template parameter and argument
lets['a'] = 1;
lets['B'] = 2;
lets['c'] = 3;
lets['A'] = 4; // overwrites position 'a'
cout <<= lets; // Output: a:4 B:2 c:3
struct Comp {  // Functor
    bool operator()(char a, char b) const { return toupper(a) < toupper(b); }
};
map<char,int,Comp> lets2; // here the template parameter is sufficient
```

----------------


## Listing 24.62: The initializer list must contain "pair" elements.

Book listing [lst-0091-book.cpp](lst-0091-book.cpp):
```cpp
// https://godbolt.org/z/vW4xsMPq7
using std::pair; using std::make_pair;
namespace literal_p { // better to put user-defined literals in a namespace
constexpr pair<char,char> operator "" _p(const char* s, size_t len) {
    return len>=2 ? make_pair(s[0], s[1]) : make_pair( '-', '-' );
} }
struct Q {
    char a_; int n_;
    Q(char a, int n) : a_{a}, n_{n} {}
    operator pair<const char,int>() { return make_pair(a_, n_); }
};
// …
// explicit pairs:
map<int,int> nums { pair<int,int>(3,4), make_pair(7,8), make_pair(11,23) };
map nums2 { pair<int,int>(6,1), make_pair(5,2) };
// implicit pairs from initializer lists:
map<int,char> numch{{1,'a'},{2,'b'},{3,'c'}};
map<int,int> nums3 { {7,2}, {9,4} };
using namespace literal_p;
map<char,char> pmap { "ab"_p, "cd"_p, "ef"_p }; // detour via custom literal
cout <<= pmap;                                  // Output: a:b c:d e:f
map<char,int> qmap{Q('a',1),Q('b',2),Q('c',3)}; // implicit conversions
cout <<= qmap;                                  // Output: a:1 b:2 c:3
```

Godbolt Listing [lst-0091-godb.cpp](lst-0091-godb.cpp), [https://godbolt.org/z/vW4xsMPq7](https://godbolt.org/z/vW4xsMPq7):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/vW4xsMPq7
using std::pair; using std::make_pair;
namespace literal_p { // better to put user-defined literals in a namespace
constexpr pair<char,char> operator "" _p(const char* s, size_t len) {
    return len>=2 ? make_pair(s[0], s[1]) : make_pair( '-', '-' );
} }
struct Q {
    char a_; int n_;
    Q(char a, int n) : a_{a}, n_{n} {}
    operator pair<const char,int>() { return make_pair(a_, n_); }
};
// …
// explicit pairs:
map<int,int> nums { pair<int,int>(3,4), make_pair(7,8), make_pair(11,23) };
map nums2 { pair<int,int>(6,1), make_pair(5,2) };
// implicit pairs from initializer lists:
map<int,char> numch{{1,'a'},{2,'b'},{3,'c'}};
map<int,int> nums3 { {7,2}, {9,4} };
using namespace literal_p;
map<char,char> pmap { "ab"_p, "cd"_p, "ef"_p }; // detour via custom literal
cout <<= pmap;                                  // Output: a:b c:d e:f
map<char,int> qmap{Q('a',1),Q('b',2),Q('c',3)}; // implicit conversions
cout <<= qmap;                                  // Output: a:1 b:2 c:3
```

----------------


## Listing 24.63: You specify a single new element as a pair.

Book listing [lst-0092-book.cpp](lst-0092-book.cpp):
```cpp
// https://godbolt.org/z/Y1ebrxb8T
map<int,string> zip2plc;
zip2plc.insert(std::make_pair(94103, "San Francisco"));
zip2plc.emplace(10001, "New York");
cout <<= zip2plc;    // Output: 10001:New York 94103:San Francisco
map<string,int> plc2zip;
plc2zip.emplace("New York", 10001);
plc2zip.emplace("New York", 10002);  // does not overwrite
cout <<= plc2zip;                    // Output: New York:10001
```

Godbolt Listing [lst-0092-godb.cpp](lst-0092-godb.cpp), [https://godbolt.org/z/Y1ebrxb8T](https://godbolt.org/z/Y1ebrxb8T):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Y1ebrxb8T
map<int,string> zip2plc;
zip2plc.insert(std::make_pair(94103, "San Francisco"));
zip2plc.emplace(10001, "New York");
cout <<= zip2plc;    // Output: 10001:New York 94103:San Francisco
map<string,int> plc2zip;
plc2zip.emplace("New York", 10001);
plc2zip.emplace("New York", 10002);  // does not overwrite
cout <<= plc2zip;                    // Output: New York:10001
```

----------------


## Listing 24.64: Automatically create and immediately overwrite with "operator[]".

Book listing [lst-0093-book.cpp](lst-0093-book.cpp):
```cpp
// https://godbolt.org/z/7h3vsWvYG 
map<string,int> dwarves;
dwarves.emplace("Fili",  2859);
cout << dwarves["Fili"] << '\n'; // Output: 2859
cout << dwarves["Dori"] << '\n'; // newly created. Output: 0
dwarves["Kili"] = 2846;          // newly created and immediately overwritten
cout << dwarves["Kili"] << '\n'; // Output: 2846
cout <<= dwarves;                // Output: Dori:0 Fili:2859 Kili:2846
```

Godbolt Listing [lst-0093-godb.cpp](lst-0093-godb.cpp), [https://godbolt.org/z/7h3vsWvYG](https://godbolt.org/z/7h3vsWvYG):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7h3vsWvYG 
map<string,int> dwarves;
dwarves.emplace("Fili",  2859);
cout << dwarves["Fili"] << '\n'; // Output: 2859
cout << dwarves["Dori"] << '\n'; // newly created. Output: 0
dwarves["Kili"] = 2846;          // newly created and immediately overwritten
cout << dwarves["Kili"] << '\n'; // Output: 2846
cout <<= dwarves;                // Output: Dori:0 Fili:2859 Kili:2846
```

----------------


## Listing 24.65: You can change the value of a target.

Book listing [lst-0094-book.cpp](lst-0094-book.cpp):
```cpp
// https://godbolt.org/z/8948besY8
map<string,string> data { {"John","Wayne"}, {"Cary","Grant" }, };
cout <<= data;                         // Cary:Grant John:Wayne
data.rbegin()->second = "Travolta";    // Overwrite target
cout <<= data;                         // Cary:Grant John:Travolta
```

Godbolt Listing [lst-0094-godb.cpp](lst-0094-godb.cpp), [https://godbolt.org/z/8948besY8](https://godbolt.org/z/8948besY8):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8948besY8
map<string,string> data { {"John","Wayne"}, {"Cary","Grant" }, };
cout <<= data;                         // Cary:Grant John:Wayne
data.rbegin()->second = "Travolta";    // Overwrite target
cout <<= data;                         // Cary:Grant John:Travolta
```

----------------


## Listing 24.66: Iterators of "map" are of type "pair".

Book listing [lst-0095-book.cpp](lst-0095-book.cpp):
```cpp
// https://godbolt.org/z/16T5dob75 
map<char,int> data { { 'a',1}, {'b',2}, {'c',3} };
for(auto it=data.rbegin(); it!=data.rend(); ++it) { // backwards
  cout << it->first << ':' << it->second << ' ';    // dereference with ->
}
cout << '\n'; // Output: c:3 b:2 a:1
for(auto &e : data) {                        // forwards, uses begin() and end()
  cout << e.first << ':' << e.second << ' '; // pair, element access with .
}
cout << '\n'; // Output: a:1 b:2 c:3
```

Godbolt Listing [lst-0095-godb.cpp](lst-0095-godb.cpp), [https://godbolt.org/z/16T5dob75](https://godbolt.org/z/16T5dob75):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/16T5dob75 
map<char,int> data { { 'a',1}, {'b',2}, {'c',3} };
for(auto it=data.rbegin(); it!=data.rend(); ++it) { // backwards
  cout << it->first << ':' << it->second << ' ';    // dereference with ->
}
cout << '\n'; // Output: c:3 b:2 a:1
for(auto &e : data) {                        // forwards, uses begin() and end()
  cout << e.first << ':' << e.second << ' '; // pair, element access with .
}
cout << '\n'; // Output: a:1 b:2 c:3
```

----------------


## Listing 24.67: You cannot use "operator[]" on a "const map".

Book listing [lst-0096-book.cpp](lst-0096-book.cpp):
```cpp
// https://godbolt.org/z/Eqsjex4Ta
string search7(const map<int,string> &data) {
    return data[7];               // (ERR) non-const method on const parameter
}
string search5(const map<int,string> &data) {
    auto it = data.find(5);       // not automatically inserting
    return it==data.end() ? string{} : it->second;
}
// …
map<int,string> dwarfs{ {1,"one"}, {3,"three"}, {5,"five"}, {7,"seven"} };
cout << search7(dwarfs) << '\n';
cout << search5(dwarfs) << '\n';  // Output: five
```

Godbolt Listing [lst-0096-godb.cpp](lst-0096-godb.cpp), [https://godbolt.org/z/Eqsjex4Ta](https://godbolt.org/z/Eqsjex4Ta):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Eqsjex4Ta
string search7(const map<int,string> &data) {
    return data[7];               // (ERR) non-const method on const parameter
}
string search5(const map<int,string> &data) {
    auto it = data.find(5);       // not automatically inserting
    return it==data.end() ? string{} : it->second;
}
// …
map<int,string> dwarfs{ {1,"one"}, {3,"three"}, {5,"five"}, {7,"seven"} };
cout << search7(dwarfs) << '\n';
cout << search5(dwarfs) << '\n';  // Output: five
```

----------------


## Listing 24.68: This is the template for the example listings in this section on "multiset".

Book listing [lst-0097-book.cpp](lst-0097-book.cpp):
```cpp
// https://godbolt.org/z/aMj87neG6
#include <set>     // multiset
#include <iostream>
using std::multiset; using std::cout;
template<typename Elem, typename Comp>
std::ostream& operator<<=(std::ostream&os, const multiset<Elem,Comp>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0097-godb.cpp](lst-0097-godb.cpp), [https://godbolt.org/z/aMj87neG6](https://godbolt.org/z/aMj87neG6):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/aMj87neG6
#include <set>     // multiset
#include <iostream>
using std::multiset; using std::cout;
template<typename Elem, typename Comp>
std::ostream& operator<<=(std::ostream&os, const multiset<Elem,Comp>&data) {
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.69: Entries are sorted, and duplicates are retained.

Book listing [lst-0098-book.cpp](lst-0098-book.cpp):
```cpp
// https://godbolt.org/z/aqGTKdbMP
#include <vector>
// …
multiset msinit{1,2,2,3,1};                 // sorted at initialization
cout <<= msinit;        // Output: 1 1 2 2 3
std::vector in{ 7,7,7,7,7,7,7 };
std::set srange( in.begin(), in.end() );    // set removes duplicates
cout << srange.size() << ": " << *srange.begin() << '\n'; // Output: 1: 7
multiset msrange( in.begin(), in.end() );   // multiset retains duplicates
cout <<= msrange;       // Output: 7 7 7 7 7 7 7
```

Godbolt Listing [lst-0098-godb.cpp](lst-0098-godb.cpp), [https://godbolt.org/z/aqGTKdbMP](https://godbolt.org/z/aqGTKdbMP):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/aqGTKdbMP
#include <vector>
// …
multiset msinit{1,2,2,3,1};                 // sorted at initialization
cout <<= msinit;        // Output: 1 1 2 2 3
std::vector in{ 7,7,7,7,7,7,7 };
std::set srange( in.begin(), in.end() );    // set removes duplicates
cout << srange.size() << ": " << *srange.begin() << '\n'; // Output: 1: 7
multiset msrange( in.begin(), in.end() );   // multiset retains duplicates
cout <<= msrange;       // Output: 7 7 7 7 7 7 7
```

----------------


## Listing 24.70: The "multiset" search functions find the range of matching elements.

Book listing [lst-0099-book.cpp](lst-0099-book.cpp):
```cpp
// https://godbolt.org/z/rWGrhbYa7
#include <string>
#include <iterator> // distance
#include <ranges>   // subrange
struct Person {
  std::string name;
  friend bool operator<(const Person &a, const Person &b) {  
    // only first letter
    return a.name.size()==0 ? true
      : (b.name.size()==0 ? false : a.name[0] < b.name[0]);
  }
};
// …
multiset data{ 1, 4,4, 2,2,2, 7, 9 };
auto [from1, to1] = data.equal_range(2);
cout << "Number of 2s: "
  << std::distance(from1, to1) << '\n';     // Output: Number of 2s: 3
auto [from2, to2] = data.equal_range(5);
cout << "Number of 5s: "
  << std::distance(from2, to2) << '\n';     // Output: Number of 5s: 0
multiset<Person> room{
  {"Karl"}, {"Kurt"}, {"Peter"}, {"Karl"}, {"Ken"}};
auto [p, q] = room.equal_range(Person{"K"});
for(auto& p : std::ranges::subrange(p,q)) { // C++20 range or simple loop
  cout << p.name << ' ';
}
cout << '\n'; // Output: Karl Kurt Karl Ken
```

Godbolt Listing [lst-0099-godb.cpp](lst-0099-godb.cpp), [https://godbolt.org/z/rWGrhbYa7](https://godbolt.org/z/rWGrhbYa7):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/rWGrhbYa7
#include <string>
#include <iterator> // distance
#include <ranges>   // subrange
struct Person {
  std::string name;
  friend bool operator<(const Person &a, const Person &b) {  
    // only first letter
    return a.name.size()==0 ? true
      : (b.name.size()==0 ? false : a.name[0] < b.name[0]);
  }
};
// …
multiset data{ 1, 4,4, 2,2,2, 7, 9 };
auto [from1, to1] = data.equal_range(2);
cout << "Number of 2s: "
  << std::distance(from1, to1) << '\n';     // Output: Number of 2s: 3
auto [from2, to2] = data.equal_range(5);
cout << "Number of 5s: "
  << std::distance(from2, to2) << '\n';     // Output: Number of 5s: 0
multiset<Person> room{
  {"Karl"}, {"Kurt"}, {"Peter"}, {"Karl"}, {"Ken"}};
auto [p, q] = room.equal_range(Person{"K"});
for(auto& p : std::ranges::subrange(p,q)) { // C++20 range or simple loop
  cout << p.name << ' ';
}
cout << '\n'; // Output: Karl Kurt Karl Ken
```

----------------


## Listing 24.71: This is the template for the listings in this section on "multimap".

Book listing [lst-0100-book.cpp](lst-0100-book.cpp):
```cpp
// https://godbolt.org/z/bxW63jPor
#include <map>        // the main thing
#include <iostream>   // for output
#include <string>     // often used for key or target
using std::multimap; using std::cout; using std::string;
template<typename Key, typename Trg, typename Cmp>
std::ostream& operator<<=(std::ostream&os, const multimap<Key,Trg,Cmp>&data){
    for(auto &e : data) {
        os << e.first << ":" << e.second << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0100-godb.cpp](lst-0100-godb.cpp), [https://godbolt.org/z/bxW63jPor](https://godbolt.org/z/bxW63jPor):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/bxW63jPor
#include <map>        // the main thing
#include <iostream>   // for output
#include <string>     // often used for key or target
using std::multimap; using std::cout; using std::string;
template<typename Key, typename Trg, typename Cmp>
std::ostream& operator<<=(std::ostream&os, const multimap<Key,Trg,Cmp>&data){
    for(auto &e : data) {
        os << e.first << ":" << e.second << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.72: All entries end up in the "multimap".

Book listing [lst-0101-book.cpp](lst-0101-book.cpp):
```cpp
// https://godbolt.org/z/6no9dW64d 
multimap int2int{ std::make_pair(3,4) };  // multimap<int,int>
using namespace std::literals; // for ""s
multimap<int,string> numlang{
    {7,"seven"s}, {6,"six"s},
    {7,"siete"s}, {6,"sechs"s},
    {7,"seven"s}, {7,"yedi"s},
    {8,"eight"s} };
cout <<= numlang; // Output: 6:six 6:sechs 7:seven 7:siete 7:seven 7:yedi 8:eight
```

Godbolt Listing [lst-0101-godb.cpp](lst-0101-godb.cpp), [https://godbolt.org/z/6no9dW64d](https://godbolt.org/z/6no9dW64d):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6no9dW64d 
multimap int2int{ std::make_pair(3,4) };  // multimap<int,int>
using namespace std::literals; // for ""s
multimap<int,string> numlang{
    {7,"seven"s}, {6,"six"s},
    {7,"siete"s}, {6,"sechs"s},
    {7,"seven"s}, {7,"yedi"s},
    {8,"eight"s} };
cout <<= numlang; // Output: 6:six 6:sechs 7:seven 7:siete 7:seven 7:yedi 8:eight
```

----------------


## Listing 24.73: "insert" and "emplace" in the "multimap".

Book listing [lst-0102-book.cpp](lst-0102-book.cpp):
```cpp
// https://godbolt.org/z/dj68ej97r 
using namespace std::literals; // for ""s

multimap<int,string> numlang{};
numlang.insert( std::make_pair(7, "seven"s) );
numlang.insert( std::pair<int,string>(7, "sieben"s) );
numlang.emplace( 7, "yedi"s );
cout <<= numlang; // Output: 7:seven 7:sieben 7:yedi
```

Godbolt Listing [lst-0102-godb.cpp](lst-0102-godb.cpp), [https://godbolt.org/z/dj68ej97r](https://godbolt.org/z/dj68ej97r):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dj68ej97r 
using namespace std::literals; // for ""s

multimap<int,string> numlang{};
numlang.insert( std::make_pair(7, "seven"s) );
numlang.insert( std::pair<int,string>(7, "sieben"s) );
numlang.emplace( 7, "yedi"s );
cout <<= numlang; // Output: 7:seven 7:sieben 7:yedi
```

----------------


## Listing 24.74: "erase" with a key can delete multiple elements.

Book listing [lst-0103-book.cpp](lst-0103-book.cpp):
```cpp
// https://godbolt.org/z/TbvKTn4Px 
multimap<char,int> vals{ {'c',1}, {'c',8}, {'g',1},
    {'c',1}, {'a',7}, {'a',1}, {'c',2}, };
cout <<= vals;             // Output: a:7 a:1 c:1 c:8 c:1 c:2 g:1
vals.erase( 'c' );         // deletes all 'c's
cout <<= vals;             // Output: a:7 a:1 g:1
vals.erase(vals.begin());  // deletes only one of the 'a's
cout <<= vals;             // Output: a:1 g:1
```

Godbolt Listing [lst-0103-godb.cpp](lst-0103-godb.cpp), [https://godbolt.org/z/TbvKTn4Px](https://godbolt.org/z/TbvKTn4Px):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TbvKTn4Px 
multimap<char,int> vals{ {'c',1}, {'c',8}, {'g',1},
    {'c',1}, {'a',7}, {'a',1}, {'c',2}, };
cout <<= vals;             // Output: a:7 a:1 c:1 c:8 c:1 c:2 g:1
vals.erase( 'c' );         // deletes all 'c's
cout <<= vals;             // Output: a:7 a:1 g:1
vals.erase(vals.begin());  // deletes only one of the 'a's
cout <<= vals;             // Output: a:1 g:1
```

----------------


## Listing 24.75: Never operate unordered associative containers with a bad hash function.

Book listing [lst-0104-book.cpp](lst-0104-book.cpp):
```cpp
// https://godbolt.org/z/1MYhxb7rq
#include <set>               // set, multiset
#include <unordered_set>     // unordered_set, unordered_multiset
#include <iostream>
#include <string>
#include <chrono>            // Time measurement
using std::cout;
using namespace std::chrono;

long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}
constexpr size_t ITERATIONS = 100'000;
template<typename Cont, typename Gen>
    requires std::invocable<Gen, size_t> &&  // C++20 concept
    requires(Gen gen, size_t n) {{gen(n)} -> std::same_as<int>;} &&
    std::same_as<typename Cont::value_type,int>
void timeStuff(std::string name, Cont data, Gen genNum) {
    cout << name << "...";
    auto start = steady_clock::now();
    for(size_t idx=0; idx<ITERATIONS; ++idx) {
        data.insert( genNum(idx) );
    }
    cout << " " << millisSince(start) << " ms" << std::endl;
}
int same(size_t) { return 7; }      // always generates the same number
int scatter(size_t n) { return int(n); } // generates different numbers
struct BadHash {      // the worst possible hash function as a functor
       size_t   operator()(int) const { return 1uz; }
};

int main() {
   std::multiset<int> m{};
   timeStuff("multiset           same           ", m, &same);
   timeStuff("multiset           scatter        ", m, &scatter);
   std::set<int> s{};
   timeStuff("set                same           ", s, &same);
   timeStuff("set                scatter        ", s, &scatter);
   std::unordered_multiset<int> um{};
   timeStuff("unordered_multiset same           ", um, &same);
   timeStuff("unordered_multiset scatter        ", um, &scatter);
   std::unordered_multiset<int,BadHash> umb{};
   timeStuff("unordered_multiset same    badHash", umb, &same);
   timeStuff("unordered_multiset scatter badHash", umb, &scatter);
}
```

Godbolt Listing [lst-0104-godb.cpp](lst-0104-godb.cpp), [https://godbolt.org/z/1MYhxb7rq](https://godbolt.org/z/1MYhxb7rq):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/1MYhxb7rq
#include <set>               // set, multiset
#include <unordered_set>     // unordered_set, unordered_multiset
#include <iostream>
#include <string>
#include <chrono>            // Time measurement
using std::cout;
using namespace std::chrono;

long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}
constexpr size_t ITERATIONS = 100'000;
template<typename Cont, typename Gen>
    requires std::invocable<Gen, size_t> &&  // C++20 concept
    requires(Gen gen, size_t n) {{gen(n)} -> std::same_as<int>;} &&
    std::same_as<typename Cont::value_type,int>
void timeStuff(std::string name, Cont data, Gen genNum) {
    cout << name << "...";
    auto start = steady_clock::now();
    for(size_t idx=0; idx<ITERATIONS; ++idx) {
        data.insert( genNum(idx) );
    }
    cout << " " << millisSince(start) << " ms" << std::endl;
}
int same(size_t) { return 7; }      // always generates the same number
int scatter(size_t n) { return int(n); } // generates different numbers
struct BadHash {      // the worst possible hash function as a functor
       size_t   operator()(int) const { return 1uz; }
};

int main() {
   std::multiset<int> m{};
   timeStuff("multiset           same           ", m, &same);
   timeStuff("multiset           scatter        ", m, &scatter);
   std::set<int> s{};
   timeStuff("set                same           ", s, &same);
   timeStuff("set                scatter        ", s, &scatter);
   std::unordered_multiset<int> um{};
   timeStuff("unordered_multiset same           ", um, &same);
   timeStuff("unordered_multiset scatter        ", um, &scatter);
   std::unordered_multiset<int,BadHash> umb{};
   timeStuff("unordered_multiset same    badHash", umb, &same);
   timeStuff("unordered_multiset scatter badHash", umb, &scatter);
}
```

----------------


## Listing 24.76: Twice as many elements with a poor hash function means four times the runtime.

Book listing [lst-0105-book.cpp](lst-0105-book.cpp):
```cpp
// https://godbolt.org/z/cc3KohPrY
#include <unordered_set>     // unordered_set, unordered_multiset
#include <iostream>
#include <string>
#include <chrono>            // Time measurement
using std::cout;
using namespace std::chrono;
long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start)
        .count();
}
struct BadHash {  // the worst possible hash function as a functor
    size_t   operator()(int) const { return 1uz; }
};
void timeStuff(size_t iters) {
    std::unordered_multiset<int,BadHash> data{};
    cout << iters << "...";
    auto start = steady_clock::now();
    for(size_t idx=0; idx<iters; ++idx) {
        data.insert( (int)idx );
    }
    cout << " " << millisSince(start) << " ms" << std::endl;
}
constexpr size_t LIMIT = 20'000;
int main() {
    size_t iters = 100;
    while(iters < LIMIT) {
        timeStuff(iters);
        iters *= 2; // double
    }
}
```

Godbolt Listing [lst-0105-godb.cpp](lst-0105-godb.cpp), [https://godbolt.org/z/cc3KohPrY](https://godbolt.org/z/cc3KohPrY):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/cc3KohPrY
#include <unordered_set>     // unordered_set, unordered_multiset
#include <iostream>
#include <string>
#include <chrono>            // Time measurement
using std::cout;
using namespace std::chrono;
long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start)
        .count();
}
struct BadHash {  // the worst possible hash function as a functor
    size_t   operator()(int) const { return 1uz; }
};
void timeStuff(size_t iters) {
    std::unordered_multiset<int,BadHash> data{};
    cout << iters << "...";
    auto start = steady_clock::now();
    for(size_t idx=0; idx<iters; ++idx) {
        data.insert( (int)idx );
    }
    cout << " " << millisSince(start) << " ms" << std::endl;
}
constexpr size_t LIMIT = 20'000;
int main() {
    size_t iters = 100;
    while(iters < LIMIT) {
        timeStuff(iters);
        iters *= 2; // double
    }
}
```

----------------


## Listing 24.77: This is the template for the example listings on "unordered_set".

Book listing [lst-0106-book.cpp](lst-0106-book.cpp):
```cpp
// https://godbolt.org/z/38TbjGhn6
#include <unordered_set>
#include <iostream>
using std::unordered_set; using std::cout; using std::ostream;
template<typename Elem, typename Cmp>
ostream& operator<<=(ostream&os, const unordered_set<Elem,Cmp>&data){
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0106-godb.cpp](lst-0106-godb.cpp), [https://godbolt.org/z/38TbjGhn6](https://godbolt.org/z/38TbjGhn6):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/38TbjGhn6
#include <unordered_set>
#include <iostream>
using std::unordered_set; using std::cout; using std::ostream;
template<typename Elem, typename Cmp>
ostream& operator<<=(ostream&os, const unordered_set<Elem,Cmp>&data){
    for(auto &e : data) {
        os << e << ' ';
    }
    return os << '\n'; // '<<=' instead of '<<' for line break
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.78: An "unordered_set" with custom comparison and hash function.

Book listing [lst-0107-book.cpp](lst-0107-book.cpp):
```cpp
// https://godbolt.org/z/TY5x54efE
#include <unordered_set>
#include <iostream>
#include <vector>
#include <string>
using std::string; using std::unordered_set; using std::cout;
struct Word {
    string word_;
    size_t row_;
    Word(const string &word, size_t row)
        : word_{word}, row_{row} {}
    friend bool operator==(const Word& a, const Word &b)
        { return a.word_ == b.word_; } // ignores row
 };
 namespace std {
 template<> struct hash<Word> { // ignores row
        std::hash<string> stringHash;
        std::size_t operator()(const Word &w) const {
            return stringHash(w.word_);
        }
 }; }
 struct ExactWordHash { // includes row
     std::hash<string> sHash;
     std::hash<size_t> iHash;
     bool operator()(const Word& a) const {
         return sHash(a.word_) ^ iHash(a.row_);
     }
 };
 struct ExactWordEqual { // includes row
     bool operator()(const Word& a, const Word &b) const {
         return std::tie(a.word_, a.row_) == std::tie(b.word_, b.row_);
     }
 };
 int main() {
     std::vector input {
       Word{"a",0}, Word{"rose",0},
       Word{"is",1}, Word{"a",1}, Word{"rose",1},
       Word{"is",2}, Word{"a",2}, Word{"rose",2},  };
     // Use overloads
     unordered_set<Word> words( input.begin(), input.end() );
     cout << words.size() << '\n'; // Output: 3
     // Use custom functors
     unordered_set<Word,ExactWordHash,ExactWordEqual> poem(
          input.begin(), input.end() );
     cout << poem.size() << '\n';  // Output: 8
     // Hash as Lambda
     auto h = [](const auto &a) { return std::hash<string>{}(a.word_); };
     unordered_set<Word,decltype(h)> rose(input.begin(), input.end(), 10, h);
     cout << rose.size() << '\n';  // Output: 3
}
```

Godbolt Listing [lst-0107-godb.cpp](lst-0107-godb.cpp), [https://godbolt.org/z/TY5x54efE](https://godbolt.org/z/TY5x54efE):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/TY5x54efE
#include <unordered_set>
#include <iostream>
#include <vector>
#include <string>
using std::string; using std::unordered_set; using std::cout;
struct Word {
    string word_;
    size_t row_;
    Word(const string &word, size_t row)
        : word_{word}, row_{row} {}
    friend bool operator==(const Word& a, const Word &b)
        { return a.word_ == b.word_; } // ignores row
 };
 namespace std {
 template<> struct hash<Word> { // ignores row
        std::hash<string> stringHash;
        std::size_t operator()(const Word &w) const {
            return stringHash(w.word_);
        }
 }; }
 struct ExactWordHash { // includes row
     std::hash<string> sHash;
     std::hash<size_t> iHash;
     bool operator()(const Word& a) const {
         return sHash(a.word_) ^ iHash(a.row_);
     }
 };
 struct ExactWordEqual { // includes row
     bool operator()(const Word& a, const Word &b) const {
         return std::tie(a.word_, a.row_) == std::tie(b.word_, b.row_);
     }
 };
 int main() {
     std::vector input {
       Word{"a",0}, Word{"rose",0},
       Word{"is",1}, Word{"a",1}, Word{"rose",1},
       Word{"is",2}, Word{"a",2}, Word{"rose",2},  };
     // Use overloads
     unordered_set<Word> words( input.begin(), input.end() );
     cout << words.size() << '\n'; // Output: 3
     // Use custom functors
     unordered_set<Word,ExactWordHash,ExactWordEqual> poem(
          input.begin(), input.end() );
     cout << poem.size() << '\n';  // Output: 8
     // Hash as Lambda
     auto h = [](const auto &a) { return std::hash<string>{}(a.word_); };
     unordered_set<Word,decltype(h)> rose(input.begin(), input.end(), 10, h);
     cout << rose.size() << '\n';  // Output: 3
}
```

----------------


## Listing 24.79: These are the ways to initialize an "unordered_set".

Book listing [lst-0109-book.cpp](lst-0109-book.cpp):
```cpp
// https://godbolt.org/z/1j6PMojWr
#include <set>
template<typename Key>
std::set<Key> sorted(const unordered_set<Key> &data)
  { return std::set<Key>(data.begin(), data.end()); }
// …
// without arguments
unordered_set<int> empty{};
cout <<= empty;    // Output:
// initializer list
unordered_set data{1,1,2,2,3,3,4,4,5,5}; // duplicates are not included
cout <<= data;     // Output approximately: 5 4 3 2 1
// copy
unordered_set copy(data);
cout <<= copy;     // Output approximately: 5 4 3 2 1
// Range
auto so1 = sorted(data);
unordered_set range(std::next(so1.begin()), std::prev(so1.end()));
cout <<= range;    // Output approximately: 2 3 4
```

Godbolt Listing [lst-0109-godb.cpp](lst-0109-godb.cpp), [https://godbolt.org/z/1j6PMojWr](https://godbolt.org/z/1j6PMojWr):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/1j6PMojWr
#include <set>
template<typename Key>
std::set<Key> sorted(const unordered_set<Key> &data)
  { return std::set<Key>(data.begin(), data.end()); }
// …
// without arguments
unordered_set<int> empty{};
cout <<= empty;    // Output:
// initializer list
unordered_set data{1,1,2,2,3,3,4,4,5,5}; // duplicates are not included
cout <<= data;     // Output approximately: 5 4 3 2 1
// copy
unordered_set copy(data);
cout <<= copy;     // Output approximately: 5 4 3 2 1
// Range
auto so1 = sorted(data);
unordered_set range(std::next(so1.begin()), std::prev(so1.end()));
cout <<= range;    // Output approximately: 2 3 4
```

----------------


## Listing 24.80: Insertion into an "unordered_set".

Book listing [lst-0110-book.cpp](lst-0110-book.cpp):
```cpp
// https://godbolt.org/z/js3MeoccM
unordered_set<int> data;
auto res1 = data.insert( 5 );                    // Insertion by copy
if(res1.second) cout << "yes, 5 now inside\n";   // that works
auto res2 = data.emplace( 5 );                   // In-place insertion
if(res2.second) cout << "second 5 now inside\n"; // that doesn't work
auto res3 = data.insert(res1.first, 6 );         // with position hint
// res3 is just an iterator without bool
cout << *res3 << '\n';                           // definitely a 6
```

Godbolt Listing [lst-0110-godb.cpp](lst-0110-godb.cpp), [https://godbolt.org/z/js3MeoccM](https://godbolt.org/z/js3MeoccM):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/js3MeoccM
unordered_set<int> data;
auto res1 = data.insert( 5 );                    // Insertion by copy
if(res1.second) cout << "yes, 5 now inside\n";   // that works
auto res2 = data.emplace( 5 );                   // In-place insertion
if(res2.second) cout << "second 5 now inside\n"; // that doesn't work
auto res3 = data.insert(res1.first, 6 );         // with position hint
// res3 is just an iterator without bool
cout << *res3 << '\n';                           // definitely a 6
```

----------------


## Listing 24.81: Deleting preserves the order of the remaining elements.

Book listing [lst-0111-book.cpp](lst-0111-book.cpp):
```cpp
// https://godbolt.org/z/hee331WGe 
unordered_set nums{ 1,2,3,4,5,6,7,8,9,10 };
cout <<= nums;               // Output similar to: 9 1 2 3 4 5 6 7 8 10
auto it = nums.begin();
while(it != nums.end()) {
    if(*it % 2 == 0) {       // even number?
        it = nums.erase(it); // Remaining elements do not change order
    } else {
        ++it;
    }
}
cout <<= nums;                // Output similar to: 9 1 3 5 7
```

Godbolt Listing [lst-0111-godb.cpp](lst-0111-godb.cpp), [https://godbolt.org/z/hee331WGe](https://godbolt.org/z/hee331WGe):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/hee331WGe 
unordered_set nums{ 1,2,3,4,5,6,7,8,9,10 };
cout <<= nums;               // Output similar to: 9 1 2 3 4 5 6 7 8 10
auto it = nums.begin();
while(it != nums.end()) {
    if(*it % 2 == 0) {       // even number?
        it = nums.erase(it); // Remaining elements do not change order
    } else {
        ++it;
    }
}
cout <<= nums;                // Output similar to: 9 1 3 5 7
```

----------------


## Listing 24.82: You can access "unordered_set" by bucket.

Book listing [lst-0112-book.cpp](lst-0112-book.cpp):
```cpp
// https://godbolt.org/z/39v4aqqd1 
// Fill with 100 values
unordered_set<int> d{};
d.rehash(10);              // try to have 10 buckets
d.max_load_factor(100.0);  // 100 elements per bucket are okay
cout << "Bucket count: " << d.bucket_count() << '\n';
for(int x : std::ranges::iota_view{0,100}) { // C++20 iota(): 0,1,2,…,99
    d.insert(x);
}
// output
for(int b = d.bucket_count()-1; b>=0; --b) {
    cout << "Bucket "<<b<<":";
    for(auto it=d.begin(b); it!=d.end(b); ++it)
        cout << *it << ' ';
    cout << '\n';
}
```

Godbolt Listing [lst-0112-godb.cpp](lst-0112-godb.cpp), [https://godbolt.org/z/39v4aqqd1](https://godbolt.org/z/39v4aqqd1):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/39v4aqqd1 
// Fill with 100 values
unordered_set<int> d{};
d.rehash(10);              // try to have 10 buckets
d.max_load_factor(100.0);  // 100 elements per bucket are okay
cout << "Bucket count: " << d.bucket_count() << '\n';
for(int x : std::ranges::iota_view{0,100}) { // C++20 iota(): 0,1,2,…,99
    d.insert(x);
}
// output
for(int b = d.bucket_count()-1; b>=0; --b) {
    cout << "Bucket "<<b<<":";
    for(auto it=d.begin(b); it!=d.end(b); ++it)
        cout << *it << ' ';
    cout << '\n';
}
```

----------------


## Listing 24.83: This is the template for the example listings on "unordered_map".

Book listing [lst-0113-book.cpp](lst-0113-book.cpp):
```cpp
// https://godbolt.org/z/T5KKWx4z4 
#include <unordered_map>
#include <iostream>
using std::unordered_map; using std::cout;
template<typename K, typename T>
std::ostream& operator<<=(std::ostream&os, const unordered_map<K,T>&data) {
    for(auto &e : data) {
        os << e.first << ":" << e.second << ' ';
    }
    return os << '\n'; // with operator<<= followed by a newline
}
int main() {
   // Example code here
}
```

Godbolt Listing [lst-0113-godb.cpp](lst-0113-godb.cpp), [https://godbolt.org/z/T5KKWx4z4](https://godbolt.org/z/T5KKWx4z4):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/T5KKWx4z4 
#include <unordered_map>
#include <iostream>
using std::unordered_map; using std::cout;
template<typename K, typename T>
std::ostream& operator<<=(std::ostream&os, const unordered_map<K,T>&data) {
    for(auto &e : data) {
        os << e.first << ":" << e.second << ' ';
    }
    return os << '\n'; // with operator<<= followed by a newline
}
int main() {
   // Example code here
}
```

----------------


## Listing 24.84: A custom data type as a key in an "unordered_map".

Book listing [lst-0114-book.cpp](lst-0114-book.cpp):
```cpp
// https://godbolt.org/z/cdeGW6a3P
#include <unordered_map>
#include <iostream>
#include <string>
using std::string; using std::unordered_map; using std::cout;
struct City {
    string name_;
    explicit City(const string &name) : name_{name} {}
    auto operator<=>(const City &b) const = default;
};
struct CityHash {
     std::hash<string> sHash;
     size_t operator()(const City& a) const {
         return sHash(a.name_);
     }
};
int main() {
    unordered_map<City, string, CityHash> cty{
        {City{"San Francisco"}, "CA"},
        {City{"Austin"}, "TX"},
        {City{"Miami"}, "FL"},
    };
    cout << cty[City{"San Francisco"}] << '\n'; // Output: CA
}
```

Godbolt Listing [lst-0114-godb.cpp](lst-0114-godb.cpp), [https://godbolt.org/z/cdeGW6a3P](https://godbolt.org/z/cdeGW6a3P):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/cdeGW6a3P
#include <unordered_map>
#include <iostream>
#include <string>
using std::string; using std::unordered_map; using std::cout;
struct City {
    string name_;
    explicit City(const string &name) : name_{name} {}
    auto operator<=>(const City &b) const = default;
};
struct CityHash {
     std::hash<string> sHash;
     size_t operator()(const City& a) const {
         return sHash(a.name_);
     }
};
int main() {
    unordered_map<City, string, CityHash> cty{
        {City{"San Francisco"}, "CA"},
        {City{"Austin"}, "TX"},
        {City{"Miami"}, "FL"},
    };
    cout << cty[City{"San Francisco"}] << '\n'; // Output: CA
}
```

----------------


## Listing 24.85: You can also use only part of your object as a key.

Book listing [lst-0115-book.cpp](lst-0115-book.cpp):
```cpp
// https://godbolt.org/z/Ed88znPn9
#include <unordered_set> // unordered_multiset
#include <iostream>
#include <string>
using std::string; using std::unordered_multiset; using std::cout;
struct City {
    string name_;
    explicit City(const string &name) : name_{name} {}
    auto operator<=>(const City &b) const = default;
 };
 struct Entry { string city_; int zip_; };
 struct EqEntry {
     bool operator()(const Entry&a, const Entry&b) const {
         return a.city_==b.city_;
     }
 };
 struct HashEntry {
     std::hash<string> sHash;
     size_t operator()(const Entry& a) const {
         return sHash(a.city_);
     }
 };
int main() {
    unordered_multiset<Entry, HashEntry, EqEntry> directory{
        {Entry{"New York", 10001}},
        {Entry{"New York", 10002}},
        {Entry{"New York", 10003}},
        {Entry{"Chicago", 60601}},
        {Entry{"Chicago", 60602}},
    };
    const Entry search{"New York", 0}; // ZIP code does not matter in search
    cout << "New York has " << directory.count(search) << " ZIP codes.\n";
    cout << "The ZIP codes of New York are:\n";
    auto [where, until] = directory.equal_range(search);
    while (where != until) {
        cout << "  " << where->zip_ << '\n';
        ++where;
    }
}
```

Godbolt Listing [lst-0115-godb.cpp](lst-0115-godb.cpp), [https://godbolt.org/z/Ed88znPn9](https://godbolt.org/z/Ed88znPn9):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/Ed88znPn9
#include <unordered_set> // unordered_multiset
#include <iostream>
#include <string>
using std::string; using std::unordered_multiset; using std::cout;
struct City {
    string name_;
    explicit City(const string &name) : name_{name} {}
    auto operator<=>(const City &b) const = default;
 };
 struct Entry { string city_; int zip_; };
 struct EqEntry {
     bool operator()(const Entry&a, const Entry&b) const {
         return a.city_==b.city_;
     }
 };
 struct HashEntry {
     std::hash<string> sHash;
     size_t operator()(const Entry& a) const {
         return sHash(a.city_);
     }
 };
int main() {
    unordered_multiset<Entry, HashEntry, EqEntry> directory{
        {Entry{"New York", 10001}},
        {Entry{"New York", 10002}},
        {Entry{"New York", 10003}},
        {Entry{"Chicago", 60601}},
        {Entry{"Chicago", 60602}},
    };
    const Entry search{"New York", 0}; // ZIP code does not matter in search
    cout << "New York has " << directory.count(search) << " ZIP codes.\n";
    cout << "The ZIP codes of New York are:\n";
    auto [where, until] = directory.equal_range(search);
    while (where != until) {
        cout << "  " << where->zip_ << '\n';
        ++where;
    }
}
```

----------------


## GodboltId:cveT8Ej7q

Book listing [lst-0117-book.cpp](lst-0117-book.cpp):
```cpp
// https://godbolt.org/z/cveT8Ej7q
struct EqEntry {
    bool operator()(const Entry&a, const Entry&b) const {
        return a.city_==b.city_;
    }
};

struct HashEntry {
    std::hash<string> sHash;
    std::hash<int> iHash;
    size_t operator()(const Entry& a) const {
        return sHash(a.city_) ^ iHash(a.zip_); // (ERR) too many elements
    }
};
```

Godbolt Listing [lst-0117-godb.cpp](lst-0117-godb.cpp), [https://godbolt.org/z/cveT8Ej7q](https://godbolt.org/z/cveT8Ej7q):
```cpp
//#(compile) c++; compiler:g141; options:-O0 -std=c++23; libs:-
// https://godbolt.org/z/cveT8Ej7q
struct EqEntry {
    bool operator()(const Entry&a, const Entry&b) const {
        return a.city_==b.city_;
    }
};

struct HashEntry {
    std::hash<string> sHash;
    std::hash<int> iHash;
    size_t operator()(const Entry& a) const {
        return sHash(a.city_) ^ iHash(a.zip_); // (ERR) too many elements
    }
};
```

----------------


## Listing 24.86: These are the ways to initialize an "unordered_multiset".

Book listing [lst-0118-book.cpp](lst-0118-book.cpp):
```cpp
// https://godbolt.org/z/bvehE8hse
#include <unordered_set> // unordered_multiset
#include <vector>
#include <iostream>
using std::unordered_multiset; using std::cout; using std::ostream;
template<typename Elem>
ostream& operator<<=(ostream&os, const unordered_multiset<Elem>&data) {
    for(auto &e : data) { os << e << ' '; } return os << '\n'; }
int main() {
    // without arguments
    unordered_multiset<int> empty(1000); // initial size of the hash table
    cout <<= empty;    // Output:
    // Initialization list; duplicates are included:
    unordered_multiset data{ 1,1,2,2,3,3,4,4,5,5 };
    cout <<= data;     // Output approximately: 5 5 4 4 3 3 2 2 1 1
    // Copy
    unordered_multiset copi(data);
    cout <<= copi;     // Output approximately: 5 5 4 4 3 3 2 2 1 1
    // Range
    std::vector in{1,2,3,10,20,30,10,20,30,1,2,3};
    unordered_multiset rang(in.begin()+3, in.end()-3);
    cout <<= rang;     // Output approximately: 30 30 20 20 10 10
}
```

Godbolt Listing [lst-0118-godb.cpp](lst-0118-godb.cpp), [https://godbolt.org/z/bvehE8hse](https://godbolt.org/z/bvehE8hse):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/bvehE8hse
#include <unordered_set> // unordered_multiset
#include <vector>
#include <iostream>
using std::unordered_multiset; using std::cout; using std::ostream;
template<typename Elem>
ostream& operator<<=(ostream&os, const unordered_multiset<Elem>&data) {
    for(auto &e : data) { os << e << ' '; } return os << '\n'; }
int main() {
    // without arguments
    unordered_multiset<int> empty(1000); // initial size of the hash table
    cout <<= empty;    // Output:
    // Initialization list; duplicates are included:
    unordered_multiset data{ 1,1,2,2,3,3,4,4,5,5 };
    cout <<= data;     // Output approximately: 5 5 4 4 3 3 2 2 1 1
    // Copy
    unordered_multiset copi(data);
    cout <<= copi;     // Output approximately: 5 5 4 4 3 3 2 2 1 1
    // Range
    std::vector in{1,2,3,10,20,30,10,20,30,1,2,3};
    unordered_multiset rang(in.begin()+3, in.end()-3);
    cout <<= rang;     // Output approximately: 30 30 20 20 10 10
}
```

----------------


## Listing 24.87: For the "multi" variants, "count" makes perfect sense.

Book listing [lst-0119-book.cpp](lst-0119-book.cpp):
```cpp
// https://godbolt.org/z/a8cGxr9s9 
#include <unordered_set> // unordered_multiset
#include <iostream>
#include <string>
using std::unordered_multiset; using std::cout; using std::string;
int main() {
    const string in = "She sells sea shells by the seashore.";
    unordered_multiset<int> cs(in.begin(), in.end()); // string as container
    cout << cs.count( 's' ) << " times s\n"; // Output: 7 times s
}
```

Godbolt Listing [lst-0119-godb.cpp](lst-0119-godb.cpp), [https://godbolt.org/z/a8cGxr9s9](https://godbolt.org/z/a8cGxr9s9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a8cGxr9s9 
#include <unordered_set> // unordered_multiset
#include <iostream>
#include <string>
using std::unordered_multiset; using std::cout; using std::string;
int main() {
    const string in = "She sells sea shells by the seashore.";
    unordered_multiset<int> cs(in.begin(), in.end()); // string as container
    cout << cs.count( 's' ) << " times s\n"; // Output: 7 times s
}
```

----------------


## Listing 24.88: Adapters work with interchangeable implementations.

Book listing [lst-0120-book.cpp](lst-0120-book.cpp):
```cpp
// https://godbolt.org/z/6MKMj3Pxv
#include <stack>
void run(auto data) { /* ... */ }  // C++20, abbreviated function template
run(stack<int>{});                 // Default: uses vector<int>
run(stack<int,vector<int>>{});     // like the default
run(stack<int,list<int>>{});       // uses list<int>
```

Godbolt Listing [lst-0120-godb.cpp](lst-0120-godb.cpp), [https://godbolt.org/z/6MKMj3Pxv](https://godbolt.org/z/6MKMj3Pxv):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/6MKMj3Pxv
#include <stack>
void run(auto data) { /* ... */ }  // C++20, abbreviated function template
run(stack<int>{});                 // Default: uses vector<int>
run(stack<int,vector<int>>{});     // like the default
run(stack<int,list<int>>{});       // uses list<int>
```

----------------


## Listing 24.89: "string" is more suitable for texts than "vector<char>".

Book listing [lst-0121-book.cpp](lst-0121-book.cpp):
```cpp
// https://godbolt.org/z/P7zbrh71d
#include <vector>
#include <string>
#include <iostream>
#include <string_view>
using std::string; using std::string_view; using std::vector; using std::cout;
int get_len(string_view str) { return str.size(); } // string_view as parameter
int main() {
    string s1 = "Hello";                  // simply with string literal
    string s2{'H','e','l','l','o'};       // or with list of char
    using namespace std::literals;        // for ""s-suffix and ""sv-suffix
    auto s3 = "Hello"s;  // even simpler with real string literal
    vector<char> v1{"Hello"};             // (ERR) no vector with string literal
    vector<char> v2{'H','e','l','l','o'}; // list of char is okay
    cout << s1 << s2 << s3 << '\n';       // Output of string works
    cout << v1 << v2 << '\n';             // (ERR) vector has no output
    const auto str = "String"s;           // Stringliteral
    const auto strv = "String-View"sv;    // String-View literal
    cout << "Length of 'str' is " << get_len(str) << '\n';   // Output: … 6
    cout << "Length of 'strv' is " << get_len(strv) << '\n'; // Output: … 11
}
```

Godbolt Listing [lst-0121-godb.cpp](lst-0121-godb.cpp), [https://godbolt.org/z/P7zbrh71d](https://godbolt.org/z/P7zbrh71d):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/P7zbrh71d
#include <vector>
#include <string>
#include <iostream>
#include <string_view>
using std::string; using std::string_view; using std::vector; using std::cout;
int get_len(string_view str) { return str.size(); } // string_view as parameter
int main() {
    string s1 = "Hello";                  // simply with string literal
    string s2{'H','e','l','l','o'};       // or with list of char
    using namespace std::literals;        // for ""s-suffix and ""sv-suffix
    auto s3 = "Hello"s;  // even simpler with real string literal
    vector<char> v1{"Hello"};             // (ERR) no vector with string literal
    vector<char> v2{'H','e','l','l','o'}; // list of char is okay
    cout << s1 << s2 << s3 << '\n';       // Output of string works
    cout << v1 << v2 << '\n';             // (ERR) vector has no output
    const auto str = "String"s;           // Stringliteral
    const auto strv = "String-View"sv;    // String-View literal
    cout << "Length of 'str' is " << get_len(str) << '\n';   // Output: … 6
    cout << "Length of 'strv' is " << get_len(strv) << '\n'; // Output: … 11
}
```

----------------


## Listing 24.90: A "bitset" example.

Book listing [lst-0122-book.cpp](lst-0122-book.cpp):
```cpp
// https://godbolt.org/z/5e3c88K8j
#include <bitset>
#include <iostream>
using std::cout;
int main() {
    std::bitset<8> bits{};         // 8 bits densely packed
    bits.set(4);                   // 5th bit to 1
    cout << bits << '\n';          // 00010000
    bits.flip();                   // invert all bits
    cout << bits << '\n';          // 11101111
    bits.set();                    // set all bits to 1
    bits.flip(1);                  // invert 2nd bit
    std::cout << bits << '\n';     // 11111101
    bits.reset();                  // set all bits to 0
    bits.set(4);                   // 5th bit to 1
    cout << bits << '\n';          // 00010000
    bits.flip();                   // invert all bits
    cout << bits << '\n';          // 11101111
    bits.set();                    // set all bits to 1
    bits.flip(1);                  // invert 2nd bit
    bits.flip(6);                  // invert 7th bit
    cout << bits << '\n';          // 10111101
    // Bitwise operations
    std::bitset<8> zack("....####", 8, '.', '#');
    cout << zack << '\n';          // 00001111
    cout << (bits & zack) << '\n'; // 00001101
    cout << (bits | zack) << '\n'; // 10111111
    cout << (bits ^ zack) << '\n'; // 10110010
    // other integer types
    std::bitset<64> b(0x123456789abcdef0LL);
    cout << b << '\n';
    // 0001001000110100010101100111100010011010101111001101111011110000
    cout << std::hex << b.to_ullong() << '\n'; // convert
    // 123456789abcdef0
}
```

Godbolt Listing [lst-0122-godb.cpp](lst-0122-godb.cpp), [https://godbolt.org/z/5e3c88K8j](https://godbolt.org/z/5e3c88K8j):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/5e3c88K8j
#include <bitset>
#include <iostream>
using std::cout;
int main() {
    std::bitset<8> bits{};         // 8 bits densely packed
    bits.set(4);                   // 5th bit to 1
    cout << bits << '\n';          // 00010000
    bits.flip();                   // invert all bits
    cout << bits << '\n';          // 11101111
    bits.set();                    // set all bits to 1
    bits.flip(1);                  // invert 2nd bit
    std::cout << bits << '\n';     // 11111101
    bits.reset();                  // set all bits to 0
    bits.set(4);                   // 5th bit to 1
    cout << bits << '\n';          // 00010000
    bits.flip();                   // invert all bits
    cout << bits << '\n';          // 11101111
    bits.set();                    // set all bits to 1
    bits.flip(1);                  // invert 2nd bit
    bits.flip(6);                  // invert 7th bit
    cout << bits << '\n';          // 10111101
    // Bitwise operations
    std::bitset<8> zack("....####", 8, '.', '#');
    cout << zack << '\n';          // 00001111
    cout << (bits & zack) << '\n'; // 00001101
    cout << (bits | zack) << '\n'; // 10111111
    cout << (bits ^ zack) << '\n'; // 10110010
    // other integer types
    std::bitset<64> b(0x123456789abcdef0LL);
    cout << b << '\n';
    // 0001001000110100010101100111100010011010101111001101111011110000
    cout << std::hex << b.to_ullong() << '\n'; // convert
    // 123456789abcdef0
}
```

----------------


## GodboltId:KPP16bdK9

Book listing [lst-0123-book.cpp](lst-0123-book.cpp):
```cpp
// https://godbolt.org/z/KPP16bdK9 
#include <iostream>
#include <valarray>
using std::ostream; using std::valarray;
ostream& operator<<(ostream&os, const valarray<double>&vs) {
    os << "[";
    for(auto&v : vs) os << v << " ";
    return os << "]";
}
int main() {
    valarray a{ 1.0, 2.0, 3.0, 4.0 }; // valarray<double>
    valarray b{ 2.0, 4.0, 6.0, 8.0 };
    valarray c{ 2.5, 1.75, 0.5, 0.125 };
    valarray<double> x = ( a + b ) * c;
    std::cout << "x: " << x << "\n";  // Output: [7.5 10.5 4.5 1.5 ]
    auto y = ( a + b ) / 2;           // y is not necessarily a valarray!
    std::cout << "y: " << y << "\n";  // Output: [1.5 3 4.5 6 ]
}
```

Godbolt Listing [lst-0123-godb.cpp](lst-0123-godb.cpp), [https://godbolt.org/z/KPP16bdK9](https://godbolt.org/z/KPP16bdK9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KPP16bdK9 
#include <iostream>
#include <valarray>
using std::ostream; using std::valarray;
ostream& operator<<(ostream&os, const valarray<double>&vs) {
    os << "[";
    for(auto&v : vs) os << v << " ";
    return os << "]";
}
int main() {
    valarray a{ 1.0, 2.0, 3.0, 4.0 }; // valarray<double>
    valarray b{ 2.0, 4.0, 6.0, 8.0 };
    valarray c{ 2.5, 1.75, 0.5, 0.125 };
    valarray<double> x = ( a + b ) * c;
    std::cout << "x: " << x << "\n";  // Output: [7.5 10.5 4.5 1.5 ]
    auto y = ( a + b ) / 2;           // y is not necessarily a valarray!
    std::cout << "y: " << y << "\n";  // Output: [1.5 3 4.5 6 ]
}
```

----------------


## GodboltId:r11584Wef

Book listing [lst-0124-book.cpp](lst-0124-book.cpp):
```cpp
// https://godbolt.org/z/r11584Wef 
#include <valarray>
#include <iostream>
using namespace std;
template<typename T>
ostream& operator<<=(ostream &os, const valarray<T>& a) { // '<<=' with newline
    for(const auto &v : a) os << v << ' ';
    return os << '\n';
}
int main() {
    // … example code here …
}
```

Godbolt Listing [lst-0124-godb.cpp](lst-0124-godb.cpp), [https://godbolt.org/z/r11584Wef](https://godbolt.org/z/r11584Wef):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/r11584Wef 
#include <valarray>
#include <iostream>
using namespace std;
template<typename T>
ostream& operator<<=(ostream &os, const valarray<T>& a) { // '<<=' with newline
    for(const auto &v : a) os << v << ' ';
    return os << '\n';
}
int main() {
    // … example code here …
}
```

----------------


## GodboltId:chnj64dva

Book listing [lst-0125-book.cpp](lst-0125-book.cpp):
```cpp
// https://godbolt.org/z/chnj64dva
valarray<int> data;               // initially size 0
cout << data.size() << "\n";      // Output: 0
data.resize(100);                 // enlarged
cout << data.size() << "\n";      // Output: 100
valarray<int> data2(200);         // space for 200 values
cout << data2.size() << "\n";     // Output: 200
valarray<int> dataC(5, 20);       // twenty 5s, different from vector
cout << dataC.size() <<": dataC[6]="<< dataC[6]<< "\n"; // Output: 20: dataC[6]=5
valarray dataD{ 2, 3, 5, 7, 11 }; // valarray<int>, initialization list
cout << dataD.size() <<": dataD[3]=" <<dataD[3]<< "\n"; // Output: 5: dataD[3]=7
```

Godbolt Listing [lst-0125-godb.cpp](lst-0125-godb.cpp), [https://godbolt.org/z/chnj64dva](https://godbolt.org/z/chnj64dva):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/chnj64dva
valarray<int> data;               // initially size 0
cout << data.size() << "\n";      // Output: 0
data.resize(100);                 // enlarged
cout << data.size() << "\n";      // Output: 100
valarray<int> data2(200);         // space for 200 values
cout << data2.size() << "\n";     // Output: 200
valarray<int> dataC(5, 20);       // twenty 5s, different from vector
cout << dataC.size() <<": dataC[6]="<< dataC[6]<< "\n"; // Output: 20: dataC[6]=5
valarray dataD{ 2, 3, 5, 7, 11 }; // valarray<int>, initialization list
cout << dataD.size() <<": dataD[3]=" <<dataD[3]<< "\n"; // Output: 5: dataD[3]=7
```

----------------


## GodboltId:PeG4K7Edc

Book listing [lst-0126-book.cpp](lst-0126-book.cpp):
```cpp
// https://godbolt.org/z/PeG4K7Edc
valarray v { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 };
valarray<int> r1(v[slice(0, 4, 3)]); // Start at 0, 4 elements, step size 3
cout <<= r1;                         // Output: 1 4 7 10
valarray<int> r2(v[v > 6]);          // addressed by valarray<bool>
cout <<= r2;                         // Output: 7 8 9 10 11 12
const valarray<size_t> indirect{ 2, 2, 3, 6 };  // duplicates allowed
valarray<int> r5(v[indirect]);       // addressed by valarray<size_t>
cout <<= r5;                         // Output: 3 3 4 7
```

Godbolt Listing [lst-0126-godb.cpp](lst-0126-godb.cpp), [https://godbolt.org/z/PeG4K7Edc](https://godbolt.org/z/PeG4K7Edc):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/PeG4K7Edc
valarray v { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 };
valarray<int> r1(v[slice(0, 4, 3)]); // Start at 0, 4 elements, step size 3
cout <<= r1;                         // Output: 1 4 7 10
valarray<int> r2(v[v > 6]);          // addressed by valarray<bool>
cout <<= r2;                         // Output: 7 8 9 10 11 12
const valarray<size_t> indirect{ 2, 2, 3, 6 };  // duplicates allowed
valarray<int> r5(v[indirect]);       // addressed by valarray<size_t>
cout <<= r5;                         // Output: 3 3 4 7
```

----------------


## GodboltId:ehP47vW9o

Book listing [lst-0127-book.cpp](lst-0127-book.cpp):
```cpp
// https://godbolt.org/z/ehP47vW9o 
valarray<int> v {
  1,  2,  3,
  4,  5,  6,
  7,  8,  9,
 10, 11, 12 };
v[slice(0, 4, 3)] *= valarray<int>(v[slice(0, 4, 3)]); // square the first column
cout <<= v;  // Output: 1 2 3 16 5 6 49 8 9 100 11 12
v[slice(0, 4, 3)] = valarray<int>{1, 4, 7, 10}; // restore
valarray<int> r3(v[gslice(0, {2, 3}, {6,2})]);  // 2-D slice from the 3-D cube
cout <<= r3;                                    // Output: 1 3 5 7 9 11
valarray<char> text("now it really starts", 20);
valarray<char> caps("NIRS", 4);
valarray<size_t> idx{ 0, 4, 7, 14 };            // Indexes in text
text[idx] = caps;                               // assign indirectly
cout <<= text;                                  // Output: Now It Really Starts
```

Godbolt Listing [lst-0127-godb.cpp](lst-0127-godb.cpp), [https://godbolt.org/z/ehP47vW9o](https://godbolt.org/z/ehP47vW9o):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/ehP47vW9o 
valarray<int> v {
  1,  2,  3,
  4,  5,  6,
  7,  8,  9,
 10, 11, 12 };
v[slice(0, 4, 3)] *= valarray<int>(v[slice(0, 4, 3)]); // square the first column
cout <<= v;  // Output: 1 2 3 16 5 6 49 8 9 100 11 12
v[slice(0, 4, 3)] = valarray<int>{1, 4, 7, 10}; // restore
valarray<int> r3(v[gslice(0, {2, 3}, {6,2})]);  // 2-D slice from the 3-D cube
cout <<= r3;                                    // Output: 1 3 5 7 9 11
valarray<char> text("now it really starts", 20);
valarray<char> caps("NIRS", 4);
valarray<size_t> idx{ 0, 4, 7, 14 };            // Indexes in text
text[idx] = caps;                               // assign indirectly
cout <<= text;                                  // Output: Now It Really Starts
```

----------------


## GodboltId:hf8rqz5x5

Book listing [lst-0128-book.cpp](lst-0128-book.cpp):
```cpp
// https://godbolt.org/z/hf8rqz5x5
#include <iostream>
#include <iomanip> // setw
#include <valarray>
using namespace std;

/* Print matrix */
template<class T>
void printMatrix(ostream&os, const valarray<T>& a, size_t n) {
    for(size_t i = 0; i < (n*n); ++i) {
        os << setw(3) << a[i];        // Print value
        os << ((i+1)%n ? ' ' : '\n'); // next line?
    }
}

/* Matrix cross product */
template<class T>

valarray<T> matmult(
        const valarray<T>& a, size_t arows, size_t acols,
        const valarray<T>& b, size_t brows, size_t bcols)
{
    /* Condition: acols==brows */
    valarray<T> result(arows * bcols);
    for(size_t i = 0; i < arows; ++i) {
      for(size_t j = 0; j < bcols; ++j) {
        auto row = a[slice(acols*i, acols, 1)]; // Row
        auto col = b[slice(j, brows, bcols)];   // Column
        result[i*bcols+j] = (row*col).sum();    // Cross product of row a[i] and 
                                                // column b[j]
      }
    }
    return result;
}

int main() {
    constexpr int n = 3;
    valarray ma{1,0,-1,  2,2,-3,  3,4,0};       // 3 x 3 matrix
    valarray mb{3,4,-1,  1,-3,0,  -1,1,2};      // 3 x 3 matrix
    printMatrix(cout, ma, n);
    cout << "  -times-\n";
    printMatrix(cout, mb, n);
    cout << "  -results in-:\n";
    valarray<int> mc = matmult(ma, n,n, mb, n,n);
    printMatrix(cout, mc, n);
}
```

Godbolt Listing [lst-0128-godb.cpp](lst-0128-godb.cpp), [https://godbolt.org/z/hf8rqz5x5](https://godbolt.org/z/hf8rqz5x5):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/hf8rqz5x5
#include <iostream>
#include <iomanip> // setw
#include <valarray>
using namespace std;

/* Print matrix */
template<class T>
void printMatrix(ostream&os, const valarray<T>& a, size_t n) {
    for(size_t i = 0; i < (n*n); ++i) {
        os << setw(3) << a[i];        // Print value
        os << ((i+1)%n ? ' ' : '\n'); // next line?
    }
}

/* Matrix cross product */
template<class T>

valarray<T> matmult(
        const valarray<T>& a, size_t arows, size_t acols,
        const valarray<T>& b, size_t brows, size_t bcols)
{
    /* Condition: acols==brows */
    valarray<T> result(arows * bcols);
    for(size_t i = 0; i < arows; ++i) {
      for(size_t j = 0; j < bcols; ++j) {
        auto row = a[slice(acols*i, acols, 1)]; // Row
        auto col = b[slice(j, brows, bcols)];   // Column
        result[i*bcols+j] = (row*col).sum();    // Cross product of row a[i] and 
                                                // column b[j]
      }
    }
    return result;
}

int main() {
    constexpr int n = 3;
    valarray ma{1,0,-1,  2,2,-3,  3,4,0};       // 3 x 3 matrix
    valarray mb{3,4,-1,  1,-3,0,  -1,1,2};      // 3 x 3 matrix
    printMatrix(cout, ma, n);
    cout << "  -times-\n";
    printMatrix(cout, mb, n);
    cout << "  -results in-:\n";
    valarray<int> mc = matmult(ma, n,n, mb, n,n);
    printMatrix(cout, mc, n);
}
```

----------------

