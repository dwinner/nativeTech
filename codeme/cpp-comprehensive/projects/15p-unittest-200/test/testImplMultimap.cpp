// we build our own suit with init_unit_test_suite():
//#define BOOST_TEST_MODULE qgram
#include <boost/test/included/unit_test.hpp>
#include <boost/test/parameterized_test.hpp>
#include <memory> // shared ptr
#include <vector>
#include <string>

// private headers in lib directory
#include "impl_multimap.hpp" // under test

using namespace boost::unit_test;
using namespace std::literals::string_literals;
using std::string; using std::vector;

//____________________________________________________________________________//
// A Test Class for a Class under Test

using UnderTest = qw::impl_multimap::index_impl;

class ImplMultimapTest { // new method -> entry in suite below
public:
    ImplMultimapTest() { std::cerr << "ImplMultimapTest()\n"; } //#
    ~ImplMultimapTest() { std::cerr << "~ImplMultimapTest()\n"; } //#
    // === constants ===
    void testConstants() {
        BOOST_REQUIRE_EQUAL(UnderTest::_prefix().length(), UnderTest::_q()-1);
        BOOST_REQUIRE_EQUAL(UnderTest::_suffix().length(), UnderTest::_q()-1);
        for(size_t i = 0; i < UnderTest::_q()-1; ++i) {
            BOOST_CHECK_EQUAL(UnderTest::_prefix()[i],'^');
            BOOST_CHECK_EQUAL(UnderTest::_suffix()[i], '$');
        }
        // oder konkreter:
        BOOST_TEST(UnderTest::_q() == 3u);
        BOOST_TEST(UnderTest::_prefix() == "^^"s);
        BOOST_TEST(UnderTest::_suffix() == "$$"s);
    }

    // === qgramify ===
    void testQgramifyEmpty() {
        // arrange
        UnderTest inst{};
        // act
        auto result = inst._qgramify(""s);
        // assert
        vector<string> expected{"^^$"s, "^$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS( result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify1() {
        // arrange
        UnderTest inst{};
        // act
        auto result = inst._qgramify("a"s);
        // assert
        vector<string> expected{"^^a"s, "^a$"s, "a$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS( result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify2() {
        // arrange
        UnderTest inst{};
        // act
        auto result = inst._qgramify("ab"s);
        // assert
        vector<string> expected{"^^a"s, "^ab"s, "ab$"s, "b$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS( result.begin(), result.end(), expected.begin(), expected.end() );
    }
    void testQgramify3() {
        // arrange
        UnderTest inst{};
        // act
        auto result = inst._qgramify("abc"s);
        // assert
        vector<string> expected{"^^a"s, "^ab"s, "abc"s, "bc$"s, "c$$"s};
        BOOST_CHECK_EQUAL_COLLECTIONS( result.begin(), result.end(), expected.begin(), expected.end() );
    }

    // === add ===
    void testAdd_nodups() {
        UnderTest inst{}; // arrange
        BOOST_REQUIRE_EQUAL(inst.size(), 0u); // assert
        inst.add("", ""); // act
        BOOST_CHECK_EQUAL(inst.size(), 1u); // assert
        inst.add("ENTRY", "entry"); // act
        BOOST_CHECK_EQUAL(inst.size(), 2u); // assert
        inst.add("OTHER", "other"); // act
        BOOST_CHECK_EQUAL(inst.size(), 3u); // assert
    }

    // === add ===
    void test_getBestMatch_empty() {
        UnderTest inst{};
        auto result = inst.getBestMatch("any");
        BOOST_CHECK_EQUAL(result, ""s);
    }
    void test_getBestMatch_one() {
        // arrange
        UnderTest inst{};
        inst.add("HOLSDERTEUFEL", "holsderteufel");
        // act
        auto result = inst.getBestMatch("ROBERT");
        BOOST_CHECK_EQUAL(result, "holsderteufel"s);
    }
    void test_getBestMatch_exact() {
        // arrange
        UnderTest inst{};
        inst.add("BERLIN", "Berlin");
        inst.add("HAMBURG", "Hamburg");
        inst.add("DORTMUND", "Dortmund");
        inst.add("STUTTGART", "Stuttgart");
        inst.add("WYK", "Wyk");
        // act and assert
        BOOST_CHECK_EQUAL(inst.getBestMatch("BERLIN"), "Berlin"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("HAMBURG"), "Hamburg"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("DORTMUND"), "Dortmund"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("STUTTGART"), "Stuttgart"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("WYK"), "Wyk"s);
    }
    void test_getBestMatch_close() {
        // arrange
        UnderTest inst{};
        inst.add("BERLIN", "Berlin");
        inst.add("HAMBURG", "Hamburg");
        inst.add("DORTMUND", "Dortmund");
        inst.add("STUTTGART", "Stuttgart");
        inst.add("WYK", "Wyk");
        // act and assert
        BOOST_CHECK_EQUAL(inst.getBestMatch("BRLIN"), "Berlin"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("BURG"), "Hamburg"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("DORTDORT"), "Dortmund"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("STUTGURT"), "Stuttgart"s);
        BOOST_CHECK_EQUAL(inst.getBestMatch("WIK"), "Wyk"s);
    }
// when you add a new test method, don't forget to add it in init_unit_test_suite() below.
};

//____________________________________________________________________________//
// Parameterized Test
//
struct Param {
    string input;
    vector<string> expected;
};
const vector<Param> params {
    // input  expected
    {""s,     {"^^$"s, "^$$"s} },
    {"A"s,    {"^^A"s, "^A$"s, "A$$"s} },
    {"AB"s,   {"^^A"s, "^AB"s, "AB$"s, "B$$"s} },
    {"ACB"s,  {"^^A"s, "^AC"s, "ACB"s, "CB$"s, "B$$"s} },
    {"AAA"s,  {"^^A"s, "^AA"s, "AAA"s, "AA$"s, "A$$"s} },
};
void testQgramify(const Param& param) {
    // arrange
    UnderTest inst{};
    // act
    auto result = inst._qgramify(param.input);
    // assert
    BOOST_CHECK_EQUAL_COLLECTIONS(
        param.expected.begin(), param.expected.end(),
        result.begin(), result.end());
}

//____________________________________________________________________________//
// Suite
//

test_suite*
init_unit_test_suite( int argc, char* argv[] )
{
    auto tester = std::make_shared<ImplMultimapTest>();
    auto &ts = framework::master_test_suite();
    ts.add( BOOST_TEST_CASE( [=](){ tester->testConstants(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramifyEmpty(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify1(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify2(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->testQgramify3(); } ));
    ts.add( BOOST_PARAM_TEST_CASE( &testQgramify, params.begin(), params.end() ) );
    ts.add( BOOST_TEST_CASE( [=](){ tester->testAdd_nodups(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_empty(); } ));
    ts.add( BOOST_TEST_CASE( [=](){ tester->test_getBestMatch_one(); } ));
    ts.add( BOOST_TEST_CASE( [](){ ImplMultimapTest().test_getBestMatch_exact(); } ));
    ts.add( BOOST_TEST_CASE( [](){ ImplMultimapTest().test_getBestMatch_close(); } ));
    return nullptr;
}
