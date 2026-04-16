/*
 * Key idea:
 *
 * For lvalue expressions more complicated than names, decltype ensures that the
 * type reported is always a lvalue reference.  That is, if a lvalue
 * expression other than a name has type T, decltype reports that type as T&.
 */

int main()
{
  int x = 0;  // decltype(x) is int
              // decltype((x)) is int&
}
