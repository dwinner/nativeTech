//#define BOOST_TEST_MODULE test_fib
#define BOOST_TEST_MAIN test_main
#include <boost/test/included/unit_test.hpp>
#include <boost/test/test_tools.hpp>

// import tested stuff
int fib(int);

namespace {

using namespace boost::unit_test;

BOOST_AUTO_TEST_CASE( test_fib_low )
{
    BOOST_CHECK( fib(0) == 0 );
    BOOST_CHECK( fib(1) == 1 );
    BOOST_CHECK( fib(2) == 1 );
    BOOST_CHECK( fib(3) == 2 );
    BOOST_CHECK( fib(4) == 3 );
    BOOST_CHECK( fib(5) == 5 );
    BOOST_CHECK( fib(6) == 8 );
}

}
