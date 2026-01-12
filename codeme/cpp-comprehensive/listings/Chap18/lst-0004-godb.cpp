//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eKbK39nG1 
class Base {
public:
    int xPublic = 1;
protected:
    int xProtected = 2;
private:
    int xPrivate = 3;
};
class DerivedPublic : public Base {
    // xPublic becomes 'public'
    // xProtected becomes 'protected'
    // xPrivate is not visible here
};
class DerivedProtected : protected Base {
    // xPublic becomes 'protected'
    // xProtected becomes 'protected'
    // xPrivate is not visible here
};
class DerivedPrivate : private Base { // or if nothing is specified
    // xPublic becomes 'private'
    // xProtected becomes 'private'
    // xPrivate is not visible here
};