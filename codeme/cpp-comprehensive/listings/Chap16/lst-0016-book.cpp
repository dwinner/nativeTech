
void byVal(MyValue arg) { }
int main() {
    MyValue value1{"ABCD"};  // new instance, constructed via string
    MyValue value2{value1};  // new instance, constructed via copy
    MyValue value3 = value1; // new instance, also via copy, despite =
    byVal(value1);           // a new instance via copy
    value1 = value2;         // not a new instance, but an assignment
}