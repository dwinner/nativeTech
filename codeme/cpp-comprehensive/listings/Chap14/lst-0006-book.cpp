
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