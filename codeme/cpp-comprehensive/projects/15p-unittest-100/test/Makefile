# convenience:
TARGET:=\
	testQwort.x
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
	for trg in $< ; do \
		echo === Testing $$trg... === ; \
		./$$trg --report_level=detailed --log_level=all \
			|| exit 1 ; \
	done

%.x: %.cpp
	$(CXX) -static -o $@ $< $(CXXFLAGS) $(LDFLAGS)
