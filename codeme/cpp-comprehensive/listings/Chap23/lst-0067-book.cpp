
// https://godbolt.org/z/Tcxnr6jTM 
template<typename T>
  concept dbl_quotable = copyable<T> &&
  requires(T t) { mk_string(t) + mk_string(t); } &&
  requires(T t) {
    {mk_string(t)} -> same_as<string>;
  };
string dbl_quote(const dbl_quotable auto& val) {
  auto val2{val}; // Create a copy (for demonstration purposes only)
  return '"' + mk_string(val) + mk_string(val2) + '"';
}