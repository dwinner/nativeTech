# Listings of Chap11.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 11.1: The main header file of the qwort.hpp library.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/e5GWbbq4Y
#ifndef QWORT_H // header guard
#define QWORT_H
#include <string>
#include <memory> // unique_ptr
namespace qw { // library namespace
    int version();
    namespace impl_multimap {
        class index_impl;
    }
    class index {
        using index_impl = impl_multimap::index_impl;
    public:
        index();
        ~index() noexcept; // needed for pimpl
        index(const index&) = default;
        index(index&&) noexcept = default;
        index& operator=(const index&) = default;
        index& operator=(index&&) = default;
    public:
        void add(const std::string &arg);
        size_t size() const;
        std::string getBestMatch(const std::string& query) const;
    public:          // public for tests
        std::string normalize(std::string arg) const;
    private:
        const std::unique_ptr<index_impl> pimpl;
    };
} // namespace qw
#endif // header guard
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/e5GWbbq4Y](https://godbolt.org/z/e5GWbbq4Y):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/e5GWbbq4Y
#ifndef QWORT_H // header guard
#define QWORT_H
#include <string>
#include <memory> // unique_ptr
namespace qw { // library namespace
    int version();
    namespace impl_multimap {
        class index_impl;
    }
    class index {
        using index_impl = impl_multimap::index_impl;
    public:
        index();
        ~index() noexcept; // needed for pimpl
        index(const index&) = default;
        index(index&&) noexcept = default;
        index& operator=(const index&) = default;
        index& operator=(index&&) = default;
    public:
        void add(const std::string &arg);
        size_t size() const;
        std::string getBestMatch(const std::string& query) const;
    public:          // public for tests
        std::string normalize(std::string arg) const;
    private:
        const std::unique_ptr<index_impl> pimpl;
    };
} // namespace qw
#endif // header guard
```

----------------


## Listing 11.2: The interface class forwards all calls to the implementation class.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/TMExE8doz 
#include "qwort/qwort.hpp" // self
#include <map>
#include <algorithm>       // transform
#include <cctype>          // toupper
#include "impl_multimap.hpp"
using std::map; using std::string;

namespace qw {
    int version() {
        return 1;
    }
    // administration
    index::index()
        : pimpl{ new index_impl{} }
        { }
    index::~index() noexcept = default;
    // interface
    void index::add(const string &arg) {
        pimpl->add(normalize(arg), arg);
    }
    size_t index::size() const {
        return pimpl->size();
    }
    string index::getBestMatch(const string& query) const {
        return pimpl->getBestMatch(normalize(query));
    }
    string index::normalize(string str) const {
        using namespace std; // begin, end
        transform(begin(str), end(str), begin(str), [](char c) {
                return ::isalpha(c) ? ::toupper(c) : '#';
            });
        return str;
    }
} // namespace qw
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/TMExE8doz](https://godbolt.org/z/TMExE8doz):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TMExE8doz 
#include "qwort/qwort.hpp" // self
#include <map>
#include <algorithm>       // transform
#include <cctype>          // toupper
#include "impl_multimap.hpp"
using std::map; using std::string;

namespace qw {
    int version() {
        return 1;
    }
    // administration
    index::index()
        : pimpl{ new index_impl{} }
        { }
    index::~index() noexcept = default;
    // interface
    void index::add(const string &arg) {
        pimpl->add(normalize(arg), arg);
    }
    size_t index::size() const {
        return pimpl->size();
    }
    string index::getBestMatch(const string& query) const {
        return pimpl->getBestMatch(normalize(query));
    }
    string index::normalize(string str) const {
        using namespace std; // begin, end
        transform(begin(str), end(str), begin(str), [](char c) {
                return ::isalpha(c) ? ::toupper(c) : '#';
            });
        return str;
    }
} // namespace qw
```

----------------


## Listing 11.3: Header of the implementation class.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/ob9voovMx 
#include <string>
#include <string_view>
#include <vector>
#include <map> // multimap
namespace qw::impl_multimap {
using std::vector; using std::multimap; 
using std::string; using std::string_view;
class index_impl {
    vector<string> entries;
    multimap<string, size_t> qindex;
public:
    void add(string_view normalized, string_view original);
    string getBestMatch(string_view normalized) const;
    size_t size() const {
        return entries.size();
    }
private:
    vector<string> qgramify(string_view normalized) const;
    static constexpr size_t Q = 3;
    static const std::string PREFIX;
    static const std::string SUFFIX;
public: // test interface
    vector<string> _qgramify(string_view n) const { return qgramify(n); }
    static size_t _q() { return Q; }
    static std::string _prefix() { return PREFIX; }
    static std::string _suffix() { return SUFFIX; }
};
} // namespace qw::impl_multimap
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/ob9voovMx](https://godbolt.org/z/ob9voovMx):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ob9voovMx 
#include <string>
#include <string_view>
#include <vector>
#include <map> // multimap
namespace qw::impl_multimap {
using std::vector; using std::multimap; 
using std::string; using std::string_view;
class index_impl {
    vector<string> entries;
    multimap<string, size_t> qindex;
public:
    void add(string_view normalized, string_view original);
    string getBestMatch(string_view normalized) const;
    size_t size() const {
        return entries.size();
    }
private:
    vector<string> qgramify(string_view normalized) const;
    static constexpr size_t Q = 3;
    static const std::string PREFIX;
    static const std::string SUFFIX;
public: // test interface
    vector<string> _qgramify(string_view n) const { return qgramify(n); }
    static size_t _q() { return Q; }
    static std::string _prefix() { return PREFIX; }
    static std::string _suffix() { return SUFFIX; }
};
} // namespace qw::impl_multimap
```

----------------


## Listing 11.4: The header of the implementation class.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/hWan33Go9
#include "impl_multimap.hpp" // header for this file
#include <map>
#include <string>
#include <string_view>

namespace qw::impl_multimap {

using std::vector; using std::multimap; using std::map; using std::string;
using std::string_view; using namespace std::literals::string_literals;

void index_impl::add(string_view normalized, string_view original) {
    /* TODO: Check for existence in 'entries' */
    const auto pos = entries.size(); // Index of the new entry
    entries.push_back(string(original));
    auto qgrams = qgramify(normalized);
    for(const auto& qgram : qgrams) {
        qindex.insert( make_pair(qgram, pos) );
    }
}
string index_impl::getBestMatch(string_view normalized) const {
    auto qgrams = qgramify(normalized);
    /* hits stores which words were hit how often */
    map<size_t, size_t> hits; /* 'entries-index' to 'hit-count' */
    size_t maxhits = 0z; /* always: max(hits.second) */
    for(const auto& qgram : qgrams) {
        auto [beg, end] = qindex.equal_range(qgram);
        for(auto it=beg; it!=end; ++it) {
            hits[it->second] += 1z; /* hit-count of the entry */
            if(hits[it->second] > maxhits) { /* simpler max-search */
                maxhits = hits[it->second];
            }
        }
    }
    /* Search first entry with maxhits. Improvement possible with PrioQueue */
    for(auto const &hit : hits) {
        if(hit.second == maxhits) {
            return entries[hit.first];
        }
    }
    /* only reached if entries is empty */
    return ""s;
}
const string index_impl::PREFIX = string(Q-1, '^');
const string index_impl::SUFFIX = string(Q-1, '$');

vector<string> index_impl::qgramify(string_view normalized) const {
    auto word = PREFIX+string(normalized)+SUFFIX; /* trick for better q-grams */
    vector<string> result {};
    auto left = word.cbegin();
    auto right = std::next(word.cbegin(), Q); /* okay: |"^^"|+|"$"| => 3 */
    for( ; right <= word.end(); ++left, ++right) {
        result.emplace_back(left, right);
    }
    return result;
}
} // namespace qw::impl_multimap
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/hWan33Go9](https://godbolt.org/z/hWan33Go9):
```cpp
//#(execute) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hWan33Go9
#include "impl_multimap.hpp" // header for this file
#include <map>
#include <string>
#include <string_view>

namespace qw::impl_multimap {

using std::vector; using std::multimap; using std::map; using std::string;
using std::string_view; using namespace std::literals::string_literals;

void index_impl::add(string_view normalized, string_view original) {
    /* TODO: Check for existence in 'entries' */
    const auto pos = entries.size(); // Index of the new entry
    entries.push_back(string(original));
    auto qgrams = qgramify(normalized);
    for(const auto& qgram : qgrams) {
        qindex.insert( make_pair(qgram, pos) );
    }
}
string index_impl::getBestMatch(string_view normalized) const {
    auto qgrams = qgramify(normalized);
    /* hits stores which words were hit how often */
    map<size_t, size_t> hits; /* 'entries-index' to 'hit-count' */
    size_t maxhits = 0z; /* always: max(hits.second) */
    for(const auto& qgram : qgrams) {
        auto [beg, end] = qindex.equal_range(qgram);
        for(auto it=beg; it!=end; ++it) {
            hits[it->second] += 1z; /* hit-count of the entry */
            if(hits[it->second] > maxhits) { /* simpler max-search */
                maxhits = hits[it->second];
            }
        }
    }
    /* Search first entry with maxhits. Improvement possible with PrioQueue */
    for(auto const &hit : hits) {
        if(hit.second == maxhits) {
            return entries[hit.first];
        }
    }
    /* only reached if entries is empty */
    return ""s;
}
const string index_impl::PREFIX = string(Q-1, '^');
const string index_impl::SUFFIX = string(Q-1, '$');

vector<string> index_impl::qgramify(string_view normalized) const {
    auto word = PREFIX+string(normalized)+SUFFIX; /* trick for better q-grams */
    vector<string> result {};
    auto left = word.cbegin();
    auto right = std::next(word.cbegin(), Q); /* okay: |"^^"|+|"$"| => 3 */
    for( ; right <= word.end(); ++left, ++right) {
        result.emplace_back(left, right);
    }
    return result;
}
} // namespace qw::impl_multimap
```

----------------


## Listing 11.5: Pretty much the simplest example program of the library.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/zj91TshnY (includes all files)
#include <cstdlib>  // EXIT_SUCCESS xcv
#include <iostream> // cout
#include <vector>
#include <string>
#include "qwort/qwort.hpp"
using std::cout; using std::vector; using std::string;
int main(int argc, const char* argv[]) {
    cout << "qwort version " << qw::version() << "\n";

    /* Build index */
    qw::index myindex{};

    /* - Demo data */
    myindex.add("Germany");
    myindex.add("Greece");

    /* Generate queries */
    vector<string> args(argv+1, argv+argc); // iterator-based initialization
    for(auto &querystring : args) {
        cout << "Searching for '" << querystring << "'... ";
        const auto match = myindex.getBestMatch(querystring);
        cout << match << "\n";
    }
    return EXIT_SUCCESS;
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/zj91TshnY](https://godbolt.org/z/zj91TshnY):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/zj91TshnY (includes all files)
#include <cstdlib>  // EXIT_SUCCESS xcv
#include <iostream> // cout
#include <vector>
#include <string>
#include "qwort/qwort.hpp"
using std::cout; using std::vector; using std::string;
int main(int argc, const char* argv[]) {
    cout << "qwort version " << qw::version() << "\n";

    /* Build index */
    qw::index myindex{};

    /* - Demo data */
    myindex.add("Germany");
    myindex.add("Greece");

    /* Generate queries */
    vector<string> args(argv+1, argv+argc); // iterator-based initialization
    for(auto &querystring : args) {
        cout << "Searching for '" << querystring << "'... ";
        const auto match = myindex.getBestMatch(querystring);
        cout << match << "\n";
    }
    return EXIT_SUCCESS;
}
```

----------------

