# conventience:
TARGET:=qwort.a
all: $(TARGET)

include ../../all.mk

# declarations
INCDIR:=../../include

# standard envs:
CXXFLAGS:=-I$(INCDIR) -Wall -pedantic -std=c++1y


$(TARGET): qwort.o
	$(AR) rcs $@ $^
	ranlib $@

qwort.o: qwort.cpp $(INCDIR)/qwort/qwort.hpp
	$(CXX) -c -o $@ qwort.cpp  $(CXXFLAGS)
