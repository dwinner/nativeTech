
Keyboard& getKeyboard() {
    cout << "  getKeyboard()\n”;
    static Keyboard keyboard{}; // static local variable
    return keyboard;
}