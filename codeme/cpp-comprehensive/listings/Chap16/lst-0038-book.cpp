
auto operator<=>(const Address& re) const {
  array a{street, zip, city};
  array b{re.street, re.zip, re.city};
  return lexicographical_compare_three_way(begin(a), end(a), begin(b), end(b));
}