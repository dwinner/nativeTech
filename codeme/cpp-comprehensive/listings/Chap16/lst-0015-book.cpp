
// https://godbolt.org/z/sGan39a8z 
struct MyValue {
   static int counter;
   int number_;
   string name_;
   explicit MyValue(string name) // as before
      : number_{++counter} , name_{name}
      { cout << setw(number_) << '_'  << "Constructor " << name_ << "\n"; }
   MyValue(const MyValue &orig) // new copy constructor
      : number_{++counter} , name_{orig.name_ + "-Copy"}
      { cout << setw(number_)<<" " << "Copy Constructor " << name_ << "\n"; }
   ~MyValue() { // as before
      cout << setw(number_)<<" " << "Destructor " << name_ << "\n";
   }
};
int MyValue::counter = 0;