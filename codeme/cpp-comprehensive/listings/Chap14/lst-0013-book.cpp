
class index_impl {
// ...
public: // test interface
    vector<string> _qgramify(string_view n) const { return qgramify(n); }
    static size_t _q() { return Q; }
    static std::string _prefix() { return PREFIX; }
    static std::string _suffix() { return SUFFIX; }
};