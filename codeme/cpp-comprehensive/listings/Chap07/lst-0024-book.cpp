
// https://godbolt.org/z/58cTfvYhe 
int two() { return 2; }       // (ERR) once with int as return type…
double two() { return 2.0; }  // (ERR) … and once with double
int main() {
    int x = two();
    double y = two();
}