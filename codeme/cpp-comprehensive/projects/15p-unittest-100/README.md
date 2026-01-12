This example mainly serves the purpose of keeping a complete project ready for unit tests.
Therefore, the `test` subdirectory is of particular interest.

It is simply built with `make`, as it was manually controlled via `Makefile`s. However, the
layout of the project is roughly prepared for use with [*CMake*][1].

> You will end up with something like this:

     └── prj
         ├── CMakeLists.txt <-- (1)
         ├── include
         │   └── prj
         │       ├── header2.hpp
         │       └── header.hpp
         ├── src
         │   ├── CMakeLists.txt <-- (2)
         │   └── x.cpp
         └── test
             ├── CMakeLists.txt <-- (3)
             ├── data
             │   └── testdata.yyy
             └── testcase.cpp

[1]: http://stackoverflow.com/a/13521867/472245 "Stackoverflow: C++ project organisation (with gtest, cmake and doxygen)"