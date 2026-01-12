
// https://godbolt.org/z/17nah59eP
//… Base2 and Value2 as before …
void output(Value2 x) {         // derived class as value
    x.print(cout);
}
int main() {
    Base2 ba2{}; output(ba2);   // (ERR) ba2 cannot be converted to Value2
    Value2 va2{}; output(va2);  // outputs 10
}