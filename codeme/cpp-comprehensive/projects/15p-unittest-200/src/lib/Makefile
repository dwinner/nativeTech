# convenience:
TARGET:=libqwort.a
all: $(TARGET)

include ../../all.mk

# declarations
INCDIR:=../../include

# standard envs:
CXXFLAGS:=-I$(INCDIR) -Wall -pedantic


$(TARGET): qwort.o impl_multimap.o
	$(AR) rcs $@ $^
	ranlib $@

%.o: %.cpp $(INCDIR)/qwort/qwort.hpp impl_multimap.hpp
	$(CXX) -c -o $@ $<  $(CXXFLAGS)
