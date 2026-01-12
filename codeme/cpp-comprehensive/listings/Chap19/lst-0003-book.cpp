
// https://godbolt.org/z/fdh4n5x84 
struct Book {
   auto getTitle() { return Title{"The C++ Handbook"s}; }
   auto getAuthor() { return Author{"Torsten T. Will"s}; }
   auto turnPage() { return /* Reference to the next page */ 42; }
   auto getPage() { return "current page content"; }
   auto getLocation() { return /* Shelf number/Book number */ 73; }
};