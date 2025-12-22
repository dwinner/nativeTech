//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/sW7MexWrh 
//… Base2 and Value2 as before …
void output(Base2& x) {       // passing as a reference
    x.print(cout);
}

int main() {
    Base2 ba2{}; output(ba2);  // outputs 8
    Value2 va2{}; output(va2); // outputs 10, because the object is not copied
}