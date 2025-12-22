# Listings of Chap17.docx

This is linked summary of the listings.
You can also view it on [one page](README-onepage.md).

 * **Listing 17.1: Prohibit four of the big five with "= delete".**
    * Book: [lst-0001-book.cpp](lst-0001-book.cpp)
    * Godbolt: [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/K1714eWr9](https://godbolt.org/z/K1714eWr9)
 * **Listing 17.2: Instead of using a raw pointer, use a standard construct and do not define any of the big five operations.**
    * Book: [lst-0002-book.cpp](lst-0002-book.cpp)
    * Godbolt: [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/nbh9aozPM](https://godbolt.org/z/nbh9aozPM)
 * **Listing 17.3: In a hierarchy with virtual methods, you must define and mark the base class destructor as virtual.**
    * Book: [lst-0003-book.cpp](lst-0003-book.cpp)
    * Godbolt: [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/6vsqnWeWh](https://godbolt.org/z/6vsqnWeWh)
 * **Listing 17.4: shared_ptr always calls the correct destructor, virtual or not.**
    * Book: [lst-0004-book.cpp](lst-0004-book.cpp)
 * **Listing 17.5: An RAII character buffer class.**
    * Book: [lst-0006-book.cpp](lst-0006-book.cpp)
    * Godbolt: [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/9eeG575Wz](https://godbolt.org/z/9eeG575Wz)
 * **Listing 17.6: If "right" throws an exception, a leak occurs with "left".**
    * Book: [lst-0007-book.cpp](lst-0007-book.cpp)
    * Godbolt: [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/TK3vTv1EM](https://godbolt.org/z/TK3vTv1EM)
 * **Listing 17.7: Correct RAII for "StereoImage".**
    * Book: [lst-0008-book.cpp](lst-0008-book.cpp)
    * Godbolt: [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/cs4eEEGrf](https://godbolt.org/z/cs4eEEGrf)
 * **Listing 17.8: C API in C++ Code**
    * Book: [lst-0009-book.cpp](lst-0009-book.cpp)
    * Godbolt: [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/hG3j3PGYv](https://godbolt.org/z/hG3j3PGYv)
 * **Listing 17.9: C API with simple RAII.**
    * Book: [lst-0010-book.cpp](lst-0010-book.cpp)
    * Godbolt: [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/n7Grhh1GM](https://godbolt.org/z/n7Grhh1GM)
 * **Listing 17.10: C API with simple RAII, without throw.**
    * Book: [lst-0011-book.cpp](lst-0011-book.cpp)
    * Godbolt: [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/c8xfM1nEa](https://godbolt.org/z/c8xfM1nEa)
 * **Listing 17.11: Nothrow-new does not throw "bad_alloc", but returns "nullptr".**
    * Book: [lst-0012-book.cpp](lst-0012-book.cpp)
    * Godbolt: [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/s8KoE4ETs](https://godbolt.org/z/s8KoE4ETs)
