
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