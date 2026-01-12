# conventience:
TARGET:=testQwort.x
all: $(TARGET)

include ../all.mk

# declarations
LIBDIR:=../src/lib
LIB:=qwort
INCDIR:=../include

# standard envs:
CXXFLAGS:=-I$(INCDIR) -Wall -pedantic -std=c++1y
LDFLAGS:=$(LIBDIR)/$(LIB).a

unittest: check

check: $(TARGET)
	./$(TARGET)

$(TARGET): testQwort.cpp
	$(CXX) -static -o $@ $< $(CXXFLAGS) $(LDFLAGS)


