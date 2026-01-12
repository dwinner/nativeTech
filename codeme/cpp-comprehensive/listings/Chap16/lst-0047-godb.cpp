//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Yjbzrrnob
enum class Weekday {
    MO=1, TU, WE, TH, FR, SA, SU         // TU becomes 2, WE becomes 3, etc.
};
enum class Level {
   TRACE=1, DEBUG, INFO=10, ERROR, FATAL // gaps are also possible
};
void log(Level level) {
   int intLevel = (int)level;            // explicitly cast to an int
   if(intLevel > 10) { /* ... */ }
}