
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