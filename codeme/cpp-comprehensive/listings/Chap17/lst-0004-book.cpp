
int main() {
    shared_ptr<Base> obj{ new Derived{} };
    /* ... more lines of code here ... */
} // obj is correctly cleaned up