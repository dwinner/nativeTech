# Listings of Chap16.docx

This is linked summary of the listings.
You can also view it on [one page](README-onepage.md).

 * **Listing 16.1: Which variables can you use for "x1" to "x5"?**
    * Book: [lst-0001-book.cpp](lst-0001-book.cpp)
    * Godbolt: [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/9jGKPbzqo](https://godbolt.org/z/9jGKPbzqo)
 * **Listing 16.2: When you call a function with a parameter for which the compiler invokes a constructor to perform a conversion, it creates a temp-value.**
    * Book: [lst-0002-book.cpp](lst-0002-book.cpp)
    * Godbolt: [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/eK1Ervjs5](https://godbolt.org/z/eK1Ervjs5)
 * **Listing 16.3: A destructor is executed when an object is removed.**
    * Book: [lst-0003-book.cpp](lst-0003-book.cpp)
    * Godbolt: [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/WPcTv3zKM](https://godbolt.org/z/WPcTv3zKM)
 * **Listing 16.4: Here, many objects are created and destroyed.**
    * Book: [lst-0004-book.cpp](lst-0004-book.cpp)
    * Godbolt: [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/ennEoWPzb](https://godbolt.org/z/ennEoWPzb)
 * **Listing 16.5: This output shows when objects are created and destroyed.**
    * Book: [lst-0005-book.cpp](lst-0005-book.cpp)
 * **Listing 16.6: A simple example of a C interface to a resource.**
    * Book: [lst-0006-book.cpp](lst-0006-book.cpp)
    * Godbolt: [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/54dzsb31z](https://godbolt.org/z/54dzsb31z)
 * **Listing 16.7: If you need to close a resource, the destructor is suitable for this.**
    * Book: [lst-0007-book.cpp](lst-0007-book.cpp)
    * Godbolt: [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/6399qK9Ts](https://godbolt.org/z/6399qK9Ts)
 * **Listing 16.8: The constructor initializes or throws an exception.**
    * Book: [lst-0008-book.cpp](lst-0008-book.cpp)
 * **Listing 16.9: Examples of Yoda conditions with "==".**
    * Book: [lst-0010-book.cpp](lst-0010-book.cpp)
 * **Listing 16.10: A Yoda condition with a method call.**
    * Book: [lst-0011-book.cpp](lst-0011-book.cpp)
 * **Listing 16.11: The constructor of "CanThrow" can terminate with an exception.**
    * Book: [lst-0012-book.cpp](lst-0012-book.cpp)
    * Godbolt: [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/99PPn595E](https://godbolt.org/z/99PPn595E)
 * **Listing 16.12: Partially initialized data fields are also cleaned up in the case of an exception.**
    * Book: [lst-0013-book.cpp](lst-0013-book.cpp)
 * **Listing 16.13: Passing by value creates objects with the compiler-generated copy constructor.**
    * Book: [lst-0014-book.cpp](lst-0014-book.cpp)
    * Godbolt: [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/3T869arc8](https://godbolt.org/z/3T869arc8)
 * **Listing 16.14: The copy constructor takes a constant reference of the class as an argument.**
    * Book: [lst-0015-book.cpp](lst-0015-book.cpp)
    * Godbolt: [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/sGan39a8z](https://godbolt.org/z/sGan39a8z)
 * **Listing 16.15: Copy and assignment.**
    * Book: [lst-0016-book.cpp](lst-0016-book.cpp)
 * **GodboltId:5M8xKabjK**
    * Book: [lst-0017-book.cpp](lst-0017-book.cpp)
    * Godbolt: [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/5M8xKabjK](https://godbolt.org/z/5M8xKabjK)
 * **Listing 16.16: The schema for implementing a custom assignment operator.**
    * Book: [lst-0019-book.cpp](lst-0019-book.cpp)
    * Godbolt: [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/s65GvY9xj](https://godbolt.org/z/s65GvY9xj)
 * **Listing 16.17: The "const" data element is initially without an assignment operator.**
    * Book: [lst-0020-book.cpp](lst-0020-book.cpp)
 * **Listing 16.18: Use "= delete" to manually remove operations.**
    * Book: [lst-0021-book.cpp](lst-0021-book.cpp)
    * Godbolt: [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/398GhcbaE](https://godbolt.org/z/398GhcbaE)
 * **Listing 16.19: With the deleted functions, the compiler prevents incorrect usage of the class.**
    * Book: [lst-0022-book.cpp](lst-0022-book.cpp)
    * Godbolt: [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/vb5a3EjcY](https://godbolt.org/z/vb5a3EjcY)
 * **Listing 16.20: The class probably contains large amounts of data that are expensive to copy. But what is being copied here?**
    * Book: [lst-0023-book.cpp](lst-0023-book.cpp)
    * Godbolt: [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/75Kh5Ws45](https://godbolt.org/z/75Kh5Ws45)
 * **Listing 16.21: Implementation of the two move operations.**
    * Book: [lst-0024-book.cpp](lst-0024-book.cpp)
    * Godbolt: [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/aoqW86vs8](https://godbolt.org/z/aoqW86vs8)
 * **Listing 16.22: Infix, function, and method notation for operators.**
    * Book: [lst-0030-book.cpp](lst-0030-book.cpp)
    * Godbolt: [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/P99563KoP](https://godbolt.org/z/P99563KoP)
 * **Listing 16.23: A "friend" function is not a method, even if it is inside the class or defined there.**
    * Book: [lst-0031-book.cpp](lst-0031-book.cpp)
    * Godbolt: [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/Wvr3aK75G](https://godbolt.org/z/Wvr3aK75G)
 * **Listing 16.24: A data type almost fully equipped with all operators.**
    * Book: [lst-0032-book.cpp](lst-0032-book.cpp)
    * Godbolt: [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/YonbGKj8M](https://godbolt.org/z/YonbGKj8M)
 * **Listing 16.25: You use the data type equipped with operators as usual.**
    * Book: [lst-0033-book.cpp](lst-0033-book.cpp)
    * Godbolt: [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/YonbGKj8M](https://godbolt.org/z/YonbGKj8M)
 * **Listing 16.26: This data type demonstrates the Boolean operators.**
    * Book: [lst-0034-book.cpp](lst-0034-book.cpp)
    * Godbolt: [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/vjrr6d5TW](https://godbolt.org/z/vjrr6d5TW)
 * **Listing 16.27: The comparison categories contain constants for the return.**
    * Book: [lst-0036-book.cpp](lst-0036-book.cpp)
    * Godbolt: [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/KxWb4q8q9](https://godbolt.org/z/KxWb4q8q9)
 * **Listing 16.28: You can use this helper function to implement a simple ordered comparison.**
    * Book: [lst-0038-book.cpp](lst-0038-book.cpp)
 * **Listing 16.29: A virtual method that is = 0 is called pure virtual or abstract.**
    * Book: [lst-0039-book.cpp](lst-0039-book.cpp)
    * Godbolt: [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/1zq4jfdYb](https://godbolt.org/z/1zq4jfdYb)
 * **Listing 16.30: With an "enum", you define a type with its own value range.**
    * Book: [lst-0044-book.cpp](lst-0044-book.cpp)
    * Godbolt: [lst-0044-godb.cpp](lst-0044-godb.cpp), [https://godbolt.org/z/xhd7s1YhE](https://godbolt.org/z/xhd7s1YhE)
 * **GodboltId:f8on7bs1Y**
    * Book: [lst-0046-book.cpp](lst-0046-book.cpp)
    * Godbolt: [lst-0046-godb.cpp](lst-0046-godb.cpp), [https://godbolt.org/z/f8on7bs1Y](https://godbolt.org/z/f8on7bs1Y)
 * **Listing 17.31: In an "enum", you can also specify the desired numerical values.**
    * Book: [lst-0047-book.cpp](lst-0047-book.cpp)
    * Godbolt: [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/Yjbzrrnob](https://godbolt.org/z/Yjbzrrnob)
