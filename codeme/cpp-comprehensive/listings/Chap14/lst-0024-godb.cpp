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