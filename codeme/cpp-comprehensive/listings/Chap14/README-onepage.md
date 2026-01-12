# Listings of Chap14.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 14.1: Solitary tests test only one component, and helper classes are substituted.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/a83sf9h8E 
#include <iostream>
// Production code:
struct DatabaseInterface {
    virtual int getData() const = 0;
};
struct Program {
    DatabaseInterface &db_;
    void run() {
        std:: cout << db_.getData() << "\n”;
    }
};
// Test helper:
struct MockDatabase : public DatabaseInterface {
    int getData() const override { return 5; }
};
// main as test:
int main() {
    MockDatabase mockDb;
    Program prog { mockDb }; // real DB is not tested
    prog.run();                // Expected output: 5
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/a83sf9h8E](https://godbolt.org/z/a83sf9h8E):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a83sf9h8E 
#include <iostream>
// Production code:
struct DatabaseInterface {
    virtual int getData() const = 0;
};
struct Program {
    DatabaseInterface &db_;
    void run() {
        std:: cout << db_.getData() << "\n”;
    }
};
// Test helper:
struct MockDatabase : public DatabaseInterface {
    int getData() const override { return 5; }
};
// main as test:
int main() {
    MockDatabase mockDb;
    Program prog { mockDb }; // real DB is not tested
    prog.run();                // Expected output: 5
}
```

----------------


## Listing 14.2: Social tests test the interaction of components.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/fsroE4seh 
#include <iostream>

// Production code:
struct DatabaseInterface {
    virtual int getData() const = 0;
};
struct RealDatabase : public DatabaseInterface {
    int getData() const override { return 999; }
};
struct Program {
    DatabaseInterface &db_;
    void run() {
        std:: cout << db_.getData() << "\n";
    }
};

// main as test:
int main() {
    RealDatabase db;
    Program prog { db }; // real DB is tested along
    prog.run();            // Expected output: 999
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/fsroE4seh](https://godbolt.org/z/fsroE4seh):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fsroE4seh 
#include <iostream>

// Production code:
struct DatabaseInterface {
    virtual int getData() const = 0;
};
struct RealDatabase : public DatabaseInterface {
    int getData() const override { return 999; }
};
struct Program {
    DatabaseInterface &db_;
    void run() {
        std:: cout << db_.getData() << "\n";
    }
};

// main as test:
int main() {
    RealDatabase db;
    Program prog { db }; // real DB is tested along
    prog.run();            // Expected output: 999
}
```

----------------


## GodboltId:sTaoeqfxr

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/sTaoeqfxr 
void testDoubleIt5() {
    // arrange
    auto param = 5;
    // act
    auto result = doubleIt(param);
    // assert
    assertTrue(result == 10);
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/sTaoeqfxr](https://godbolt.org/z/sTaoeqfxr):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/sTaoeqfxr 
void testDoubleIt5() {
    // arrange
    auto param = 5;
    // act
    auto result = doubleIt(param);
    // assert
    assertTrue(result == 10);
}
```

----------------


## GodboltId:rn5oK3M6e

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/rn5oK3M6e 
void testDoubleIt() {
    assertTrue( doubleIt(0) == 0 );
    assertTrue( doubleIt(-1) == -2 );
    assertTrue( doubleIt(1) == 2 );
    assertTrue( doubleIt(5) == 10 );
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/rn5oK3M6e](https://godbolt.org/z/rn5oK3M6e):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rn5oK3M6e 
void testDoubleIt() {
    assertTrue( doubleIt(0) == 0 );
    assertTrue( doubleIt(-1) == -2 );
    assertTrue( doubleIt(1) == 2 );
    assertTrue( doubleIt(5) == 10 );
}
```

----------------


## Listing 14.3: In the "fib.cpp" file, there is a free function that you want to test.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/Ex6Mv6xcz 
unsigned fib(unsigned n) {
    if(n==0) { return 0; }
    if(n==1) { return 1; }
    unsigned a = 0;
    unsigned b = 1;
    unsigned sum = 1;
    while(n>1) {
        sum += a;
        a = b;
        b = sum;
        n -= 1;
    }
    return sum;
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/Ex6Mv6xcz](https://godbolt.org/z/Ex6Mv6xcz):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Ex6Mv6xcz 
unsigned fib(unsigned n) {
    if(n==0) { return 0; }
    if(n==1) { return 1; }
    unsigned a = 0;
    unsigned b = 1;
    unsigned sum = 1;
    while(n>1) {
        sum += a;
        a = b;
        b = sum;
        n -= 1;
    }
    return sum;
}
```

----------------


## GodboltId:erPdKo1qh

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/erPdKo1qh (uses boost library) 
#define BOOST_TEST_MAIN test_main            // generates main() in this module
#include <boost/test/included/unit_test.hpp> // framework
#include <boost/test/test_tools.hpp>         // BOOST_CHECK etc
unsigned fib(unsigned n);                    // to test
namespace {
using namespace boost::unit_test;
BOOST_AUTO_TEST_CASE( test_fib_low )         // arbitrary name of the test case
{
    BOOST_CHECK( fib(0) == 0 );               // individual assertions
    BOOST_CHECK( fib(1) == 1 );
    BOOST_CHECK( fib(2) == 1 );
    BOOST_CHECK( fib(3) == 2 );
    BOOST_CHECK( fib(4) == 3 );
    BOOST_CHECK( fib(5) == 5 );
    BOOST_CHECK( fib(6) == 8 );
}
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/erPdKo1qh](https://godbolt.org/z/erPdKo1qh):
```cpp
//#(compile) c++; compiler:g141; options:-O0 -std=c++23; libs:boost@184
// https://godbolt.org/z/erPdKo1qh (uses boost library) 
#define BOOST_TEST_MAIN test_main            // generates main() in this module
#include <boost/test/included/unit_test.hpp> // framework
#include <boost/test/test_tools.hpp>         // BOOST_CHECK etc
unsigned fib(unsigned n);                    // to test
namespace {
using namespace boost::unit_test;
BOOST_AUTO_TEST_CASE( test_fib_low )         // arbitrary name of the test case
{
    BOOST_CHECK( fib(0) == 0 );               // individual assertions
    BOOST_CHECK( fib(1) == 1 );
    BOOST_CHECK( fib(2) == 1 );
    BOOST_CHECK( fib(3) == 2 );
    BOOST_CHECK( fib(4) == 3 );
    BOOST_CHECK( fib(5) == 5 );
    BOOST_CHECK( fib(6) == 8 );
}
}
```

----------------


## Listing 14.4: The main header file of the qwort.hpp library.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/j93cYaWd4 
#ifndef QWORT_H   // Header guard
#define QWORT_H
#include <string>
#include <string_view>
#include <memory> // unique_ptr
namespace qw {    // Namespace of the library

    int version();

    namespace impl_multimap {
        class index_impl;
    }

    class index {
        using index_impl = impl_multimap::index_impl;
    public:
        index();
        ~index() noexcept; // needed for pimpl
        index(const index&) = delete;
        index(index&&) noexcept;
        index& operator=(const index&) = delete;
        index& operator=(index&&) = delete;
    public:
        void add(std::string_view arg);
        size_t size() const;
        std::string getBestMatch(std::string_view query) const;
    public:          // public for tests
        std::string normalize(std::string arg) const;
    private:
        std::unique_ptr<index_impl> pimpl;
    };
} // namespace qw
#endif // Header guard
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/j93cYaWd4](https://godbolt.org/z/j93cYaWd4):
```cpp
//#(execute) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/j93cYaWd4 
#ifndef QWORT_H   // Header guard
#define QWORT_H
#include <string>
#include <string_view>
#include <memory> // unique_ptr
namespace qw {    // Namespace of the library

    int version();

    namespace impl_multimap {
        class index_impl;
    }

    class index {
        using index_impl = impl_multimap::index_impl;
    public:
        index();
        ~index() noexcept; // needed for pimpl
        index(const index&) = delete;
        index(index&&) noexcept;
        index& operator=(const index&) = delete;
        index& operator=(index&&) = delete;
    public:
        void add(std::string_view arg);
        size_t size() const;
        std::string getBestMatch(std::string_view query) const;
    public:          // public for tests
        std::string normalize(std::string arg) const;
    private:
        std::unique_ptr<index_impl> pimpl;
    };
} // namespace qw
#endif // Header guard
```

----------------


## Listing 14.5: Public interface for testing only.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
class index_impl {
// ...
public: // test interface
    vector<string> _qgramify(string_view n) const { return qgramify(n); }
    static size_t _q() { return Q; }
    static std::string _prefix() { return PREFIX; }
    static std::string _suffix() { return SUFFIX; }
};
```

----------------


## Listing 14.6: This test module tests qwort.cpp.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/TacMfah8E (uses boost library) 
#include "qwort/qwort.hpp" // under test
#define BOOST_TEST_MODULE qwort
#include <boost/test/included/unit_test.hpp>
using namespace boost::unit_test;
BOOST_AUTO_TEST_CASE( version_is_1 ) {
    BOOST_TEST(qw::version() == 1);
    // the following comparison should fail, but only produce a warning:
    BOOST_WARN_EQUAL(qw::version(), 2);   // (ERR) not equal, but continues
}
BOOST_AUTO_TEST_CASE( init_size_0 ) {
    qw::index inst{};                      // arrange
    auto sz = inst.size();                 // act
    BOOST_TEST(sz == 0u);                  // assert
}
BOOST_AUTO_TEST_CASE( add_size_1 ) {
    using namespace std::literals::string_literals;
    qw::index inst{};                      // arrange
    inst.add(""s);                         // act
    BOOST_REQUIRE_EQUAL(inst.size(), 1u); // assert
}
BOOST_AUTO_TEST_CASE( normalize ) {
    using namespace std::literals::string_literals;
    qw::index inst{}; // arrange
    // acts and asserts; could also be in separate functions
    BOOST_CHECK_EQUAL(inst.normalize("a"s), "A"s);
    BOOST_CHECK_EQUAL(inst.normalize("Town"s), "TOWN"s);
    BOOST_CHECK_EQUAL(inst.normalize("White Space"s), "WHITE#SPACE"s);
    BOOST_CHECK_EQUAL(inst.normalize("!Sym-bol."s), "#SYM#BOL#"s);
}
BOOST_AUTO_TEST_CASE( move ) {
    qw::index inst{};
    qw::index other = std::move( inst );
    BOOST_CHECK_EQUAL(other.size(), 0u); // pimpl successfully moved?
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/TacMfah8E](https://godbolt.org/z/TacMfah8E):
```cpp
//#(compile) c++; compiler:g141; options:-O0 -std=c++23; libs:boost@184
// https://godbolt.org/z/TacMfah8E (uses boost library) 
#include "qwort/qwort.hpp" // under test
#define BOOST_TEST_MODULE qwort
#include <boost/test/included/unit_test.hpp>
using namespace boost::unit_test;
BOOST_AUTO_TEST_CASE( version_is_1 ) {
    BOOST_TEST(qw::version() == 1);
    // the following comparison should fail, but only produce a warning:
    BOOST_WARN_EQUAL(qw::version(), 2);   // (ERR) not equal, but continues
}
BOOST_AUTO_TEST_CASE( init_size_0 ) {
    qw::index inst{};                      // arrange
    auto sz = inst.size();                 // act
    BOOST_TEST(sz == 0u);                  // assert
}
BOOST_AUTO_TEST_CASE( add_size_1 ) {
    using namespace std::literals::string_literals;
    qw::index inst{};                      // arrange
    inst.add(""s);                         // act
    BOOST_REQUIRE_EQUAL(inst.size(), 1u); // assert
}
BOOST_AUTO_TEST_CASE( normalize ) {
    using namespace std::literals::string_literals;
    qw::index inst{}; // arrange
    // acts and asserts; could also be in separate functions
    BOOST_CHECK_EQUAL(inst.normalize("a"s), "A"s);
    BOOST_CHECK_EQUAL(inst.normalize("Town"s), "TOWN"s);
    BOOST_CHECK_EQUAL(inst.normalize("White Space"s), "WHITE#SPACE"s);
    BOOST_CHECK_EQUAL(inst.normalize("!Sym-bol."s), "#SYM#BOL#"s);
}
BOOST_AUTO_TEST_CASE( move ) {
    qw::index inst{};
    qw::index other = std::move( inst );
    BOOST_CHECK_EQUAL(other.size(), 0u); // pimpl successfully moved?
}
```

----------------


## Listing 14.7: The test suite is manually assembled here.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/r9sdhaazz (uses boost library)
/* private header from lib directory: */
#include "impl_multimap.hpp"                  // to test
// we define init_unit_test_suite() ourselves, so DO NOT set:
/* #define BOOST_TEST_MODULE qgram */
#include <boost/test/included/unit_test.hpp>
#include <memory>                             // shared ptr
#include <vector>
#include <string>
using namespace boost::unit_test;
using namespace std::literals::string_literals;
using std::string; using std::vector;
/* === A test class for the class under test === */
using UnderTest = qw::impl_multimap::index_impl;
class ImplMultimapTest {                      // class with test methods
public:
    /* === Constants === */
    void testConstants() {
        BOOST_REQUIRE_EQUAL(UnderTest::_prefix().length(), UnderTest::_q()-1);
        BOOST_REQUIRE_EQUAL(UnderTest::_suffix().length(), UnderTest::_q()-1);
        for(size_t i = 0; i < UnderTest::_q()-1; ++i) {
            BOOST_CHECK_EQUAL(UnderTest::_prefix()[i], '^');
            BOOST_CHECK_EQUAL(UnderTest::_suffix()[i], '$');
        }
        // or more specifically:
        BOOST_TEST(UnderTest::_q() == 3u);
        BOOST_TEST(UnderTest::_prefix() == "^^"s);
        BOOST_TEST(UnderTest::_suffix() == "$$"s);
    }
    /* === qgramify === */
    void testQgramifyEmpty() {
        UnderTest inst{};
        auto result = inst._qgramify(""s);
        vector<string> expected{"^^$"s, "^$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify1() {
        UnderTest inst{};
        auto result = inst._qgramify("a"s);
        vector<string> expected{"^^a"s, "^a$"s, "a$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify2() {
        UnderTest inst{};
        auto result = inst._qgramify("ab"s);
        vector<string> expected{"^^a"s, "^ab"s, "ab$"s, "b$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify3() {
        UnderTest inst{};
        auto result = inst._qgramify("abc"s);
        vector<string> expected{"^^a"s, "^ab"s, "abc"s, "bc$"s, "c$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    /* === add === */
    void testAdd_nodups() {
        UnderTest inst{};                     /* arrange */
        BOOST_REQUIRE_EQUAL(inst.size(), 0u); /* assert */
        inst.add("", "");                     /* act */
        BOOST_CHECK_EQUAL(inst.size(), 1u);   /* assert */
        inst.add("ENTRY", "entry");           /* act */
        BOOST_CHECK_EQUAL(inst.size(), 2u);   /* assert */
        inst.add("OTHER", "other");           /* act */
        BOOST_CHECK_EQUAL(inst.size(), 3u);   /* assert */
    }
    /* === add === */
    void test_getBestMatch_empty() {
        UnderTest inst{};
        auto result = inst.getBestMatch("any");
        BOOST_CHECK_EQUAL(result, ""s);
    }
    void test_getBestMatch_one() {
        /* arrange */
        UnderTest inst{};
        inst.add("HOLSDERTEUFEL", "holsderteufel");
        /* act */
        auto result = inst.getBestMatch("ROBERT");
        BOOST_CHECK_EQUAL(result, "holsderteufel"s);
    }
    void test_getBestMatch_exact() {
        /* arrange */
        UnderTest inst{};
        inst.add("BERLIN", "Berlin");
        inst.add("HAMBURG", "Hamburg");
        inst.add("DORTMUND", "Dortmund");
        inst.add("STUTTGART", "Stuttgart");
        inst.add("WYK", "Wyk");
        /* act and assert */
        BOOST_CHECK_EQUAL(inst.getBestMatch("BERLIN"), "Berlin"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("HAMBURG"), "Hamburg"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("DORTMUND"), "Dortmund"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("STUTTGART"), "Stuttgart"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("WYK"), "Wyk"s);
    }
    void test_getBestMatch_close() {
        /* arrange */
        UnderTest inst{};
        inst.add("BERLIN", "Berlin");
        inst.add("HAMBURG", "Hamburg");
        inst.add("DORTMUND", "Dortmund");
        inst.add("STUTTGART", "Stuttgart");
        inst.add("WYK", "Wyk");
        /* act and assert */
        BOOST_CHECK_EQUAL(inst.getBestMatch("BRLIN"), "Berlin"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("BURG"), "Hamburg"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("DORTDORT"), "Dortmund"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("STUTGURT"), "Stuttgart"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("WIK"), "Wyk"s);
    }
/* For new test methods: Don't forget to add to init_unit_test_suite() */
};
/* === Suite === */
test_suite* init_unit_test_suite( int argc, char* argv[] ) {
    auto tester = std::make_shared<ImplMultimapTest>();
    auto &ts = framework::master_test_suite();
    ts.add( BOOST_TEST_CASE( [=](){ tester->testConstants(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramifyEmpty(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify1(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify2(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify3(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testAdd_nodups(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_empty(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_one(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_exact(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_close(); } ));
    return nullptr;
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/r9sdhaazz](https://godbolt.org/z/r9sdhaazz):
```cpp
//#(compile) c++; compiler:g141; options:-O0 -std=c++23; libs:boost@184
// https://godbolt.org/z/r9sdhaazz (uses boost library)
/* private header from lib directory: */
#include "impl_multimap.hpp"                  // to test
// we define init_unit_test_suite() ourselves, so DO NOT set:
/* #define BOOST_TEST_MODULE qgram */
#include <boost/test/included/unit_test.hpp>
#include <memory>                             // shared ptr
#include <vector>
#include <string>
using namespace boost::unit_test;
using namespace std::literals::string_literals;
using std::string; using std::vector;
/* === A test class for the class under test === */
using UnderTest = qw::impl_multimap::index_impl;
class ImplMultimapTest {                      // class with test methods
public:
    /* === Constants === */
    void testConstants() {
        BOOST_REQUIRE_EQUAL(UnderTest::_prefix().length(), UnderTest::_q()-1);
        BOOST_REQUIRE_EQUAL(UnderTest::_suffix().length(), UnderTest::_q()-1);
        for(size_t i = 0; i < UnderTest::_q()-1; ++i) {
            BOOST_CHECK_EQUAL(UnderTest::_prefix()[i], '^');
            BOOST_CHECK_EQUAL(UnderTest::_suffix()[i], '$');
        }
        // or more specifically:
        BOOST_TEST(UnderTest::_q() == 3u);
        BOOST_TEST(UnderTest::_prefix() == "^^"s);
        BOOST_TEST(UnderTest::_suffix() == "$$"s);
    }
    /* === qgramify === */
    void testQgramifyEmpty() {
        UnderTest inst{};
        auto result = inst._qgramify(""s);
        vector<string> expected{"^^$"s, "^$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify1() {
        UnderTest inst{};
        auto result = inst._qgramify("a"s);
        vector<string> expected{"^^a"s, "^a$"s, "a$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify2() {
        UnderTest inst{};
        auto result = inst._qgramify("ab"s);
        vector<string> expected{"^^a"s, "^ab"s, "ab$"s, "b$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify3() {
        UnderTest inst{};
        auto result = inst._qgramify("abc"s);
        vector<string> expected{"^^a"s, "^ab"s, "abc"s, "bc$"s, "c$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS(
            result.begin(), result.end(), expected.begin(), expected.end() );
    }
    /* === add === */
    void testAdd_nodups() {
        UnderTest inst{};                     /* arrange */
        BOOST_REQUIRE_EQUAL(inst.size(), 0u); /* assert */
        inst.add("", "");                     /* act */
        BOOST_CHECK_EQUAL(inst.size(), 1u);   /* assert */
        inst.add("ENTRY", "entry");           /* act */
        BOOST_CHECK_EQUAL(inst.size(), 2u);   /* assert */
        inst.add("OTHER", "other");           /* act */
        BOOST_CHECK_EQUAL(inst.size(), 3u);   /* assert */
    }
    /* === add === */
    void test_getBestMatch_empty() {
        UnderTest inst{};
        auto result = inst.getBestMatch("any");
        BOOST_CHECK_EQUAL(result, ""s);
    }
    void test_getBestMatch_one() {
        /* arrange */
        UnderTest inst{};
        inst.add("HOLSDERTEUFEL", "holsderteufel");
        /* act */
        auto result = inst.getBestMatch("ROBERT");
        BOOST_CHECK_EQUAL(result, "holsderteufel"s);
    }
    void test_getBestMatch_exact() {
        /* arrange */
        UnderTest inst{};
        inst.add("BERLIN", "Berlin");
        inst.add("HAMBURG", "Hamburg");
        inst.add("DORTMUND", "Dortmund");
        inst.add("STUTTGART", "Stuttgart");
        inst.add("WYK", "Wyk");
        /* act and assert */
        BOOST_CHECK_EQUAL(inst.getBestMatch("BERLIN"), "Berlin"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("HAMBURG"), "Hamburg"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("DORTMUND"), "Dortmund"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("STUTTGART"), "Stuttgart"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("WYK"), "Wyk"s);
    }
    void test_getBestMatch_close() {
        /* arrange */
        UnderTest inst{};
        inst.add("BERLIN", "Berlin");
        inst.add("HAMBURG", "Hamburg");
        inst.add("DORTMUND", "Dortmund");
        inst.add("STUTTGART", "Stuttgart");
        inst.add("WYK", "Wyk");
        /* act and assert */
        BOOST_CHECK_EQUAL(inst.getBestMatch("BRLIN"), "Berlin"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("BURG"), "Hamburg"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("DORTDORT"), "Dortmund"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("STUTGURT"), "Stuttgart"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("WIK"), "Wyk"s);
    }
/* For new test methods: Don't forget to add to init_unit_test_suite() */
};
/* === Suite === */
test_suite* init_unit_test_suite( int argc, char* argv[] ) {
    auto tester = std::make_shared<ImplMultimapTest>();
    auto &ts = framework::master_test_suite();
    ts.add( BOOST_TEST_CASE( [=](){ tester->testConstants(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramifyEmpty(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify1(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify2(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify3(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testAdd_nodups(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_empty(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_one(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_exact(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_close(); } ));
    return nullptr;
}
```

----------------


## Listing 14.8: A test function with a parameter can be called with test data.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/5GondGsMz (uses boost library)
#include <boost/test/parameterized_test.hpp>
struct Param {
    string input;
    vector<string> expected;
};
const vector<Param> params {
    // { input, expected result }
    {""s,     {"^^$"s, "^$$"s} },
    {"A"s,    {"^^A"s, "^A$"s, "A$$"s} },
    {"AB"s,   {"^^A"s, "^AB"s, "AB$"s, "B$$"s} },
    {"ACB"s,  {"^^A"s, "^AC"s, "ACB"s, "CB$"s, "B$$"s} },
    {"AAA"s,  {"^^A"s, "^AA"s, "AAA"s, "AA$"s, "A$$"s} },
};
void testQgramify(const Param& param) {
    /* arrange */
    UnderTest inst{};
    /* act */
    auto result = inst._qgramify(param.input);
    /* assert */
    BOOST_CHECK_EQUAL_COLLECTIONS(
        param.expected.begin(), param.expected.end(),
        result.begin(), result.end());
}
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/5GondGsMz](https://godbolt.org/z/5GondGsMz):
```cpp
//#(compile) c++; compiler:g141; options:-O0 -std=c++23; libs:boost@184
// https://godbolt.org/z/5GondGsMz (uses boost library)
#include <boost/test/parameterized_test.hpp>
struct Param {
    string input;
    vector<string> expected;
};
const vector<Param> params {
    // { input, expected result }
    {""s,     {"^^$"s, "^$$"s} },
    {"A"s,    {"^^A"s, "^A$"s, "A$$"s} },
    {"AB"s,   {"^^A"s, "^AB"s, "AB$"s, "B$$"s} },
    {"ACB"s,  {"^^A"s, "^AC"s, "ACB"s, "CB$"s, "B$$"s} },
    {"AAA"s,  {"^^A"s, "^AA"s, "AAA"s, "AA$"s, "A$$"s} },
};
void testQgramify(const Param& param) {
    /* arrange */
    UnderTest inst{};
    /* act */
    auto result = inst._qgramify(param.input);
    /* assert */
    BOOST_CHECK_EQUAL_COLLECTIONS(
        param.expected.begin(), param.expected.end(),
        result.begin(), result.end());
}
```

----------------

