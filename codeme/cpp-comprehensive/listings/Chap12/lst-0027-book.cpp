
struct Person {
    string name_;
    int age_;
    string city_;
    Person();          // declare constructor
};
Person::Person()
  : name_{"no name"}   // initialization value for name_
  , age_{-1}           // initialization value for age_
  , city_{"no city"}   // initialization value for city_
{ }                    // empty function body