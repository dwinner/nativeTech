# convenience:
TARGET:=query.x
all: $(TARGET)

include ../../all.mk

# declarations
LIBDIR:=../lib
LIB:=qwort
INCDIR:=../../include

# standard envs:
CXXFLAGS:=-I$(INCDIR) -Wall -pedantic -std=c++1y
LDFLAGS:=$(LIBDIR)/$(LIB).a

check: $(TARGET)
	./$(TARGET) Deutschland

$(TARGET): query.cpp
	$(CXX) -static -o $@ $< $(CXXFLAGS) $(LDFLAGS)

