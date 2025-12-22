
// https://godbolt.org/z/Tq9xbGxs4 
struct Book {
   auto getTitle() { return Title{"The C++ Handbook"s}; }
   auto getAuthor() { return Author{"Torsten T. Will"s}; }
   auto turnPage() { return /* Reference to the next page */ 42; }
   auto getPage() { return "current page content"; }
   auto getLocation() { return /* Shelf number/Book number */ 73; }
};
struct BookFinder {
    Catalog catalog;
    auto find(Book& book) { /* Shelf number/Book number */
        catalog.findBookBy(book.getTitle(), book.getAuthor());
    }
};