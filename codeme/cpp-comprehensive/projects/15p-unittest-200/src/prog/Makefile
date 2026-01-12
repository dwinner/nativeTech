# convenience:
TARGET:=query.x dictfile.x
all: $(TARGET)

include ../../all.mk

# declarations
LIBDIR:=../lib
LIB:=qwort
INCDIR:=../../include

# standard envs:
CXXFLAGS:=-I$(INCDIR) -Wall -pedantic
LDFLAGS:=-L$(LIBDIR) -l$(LIB)

check: $(TARGET)
	./query.x Germany
	./dictfile.x pg40185.txt < pg40185-query.txt


%.x: %.cpp
	$(CXX) -static -o $@ $< $(CXXFLAGS) $(LDFLAGS)
