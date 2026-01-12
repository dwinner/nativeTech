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