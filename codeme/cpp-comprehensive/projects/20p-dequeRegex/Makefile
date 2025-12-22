all:=eg.x

OBJ:=eg.o regexpr.o
CXX:=g++-6
CXXFLAGS:=-O0 -std=c++14 -Wall -Wextra -pedantic

eg.x: $(OBJ)
	$(CXX) $(LDFLAGS) -o eg.x $(OBJ)

.cpp.o:
	$(CXX) $(CXXFLAGS) -c $*.cpp

eg.o: eg.cpp regexpr.hpp

regexpr.o: regexpr.cpp regexpr.hpp

clean:
	$(RM) *.o *.x
