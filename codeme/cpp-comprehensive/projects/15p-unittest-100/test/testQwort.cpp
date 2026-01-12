#define BOOST_TEST_MODULE qwort
#include <boost/test/included/unit_test.hpp>
#include <cstdlib>

#include "qwort/qwort.hpp"

using namespace boost::unit_test;

BOOST_AUTO_TEST_CASE( version_is_1 )
{
    BOOST_CHECK_EQUAL(1, qw::version());
}
