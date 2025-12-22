
// https://godbolt.org/z/Ec7vqEdY3 
//… Base2 and Value2 as before …
void output(Base2 x) {          // pass by value
    x.print(cout);
}
int main() {
    Base2 ba2{}; output(ba2);   // outputs 8
    Value2 va2{}; output(va2);  // also outputs 8
}