# convenience:
TARGET:=\
	testImplMultimap.x \
	testQwort.x
all: $(TARGET) check

include ../all.mk

# declarations
LIBDIR:=../src/lib
LIB:=qwort
INCDIR:=../include
# - where are the Boost-Includes?
BOOSTCXXFLAGS:=-I/home/towi/src/gcc/boost/boost_1_61_0/ -Wno-placement-new
# - where are the Boost-Libs? we dont need them for these unit tests, so I have them empty
BOOSTLDFLAGS:=

# standard envs:
# - note: we add LIBDIR as include-dir because the unittests also test private stuff
CXXFLAGS:=-I$(INCDIR) $(BOOSTCXXFLAGS)  -I$(LIBDIR) -Wall -pedantic
LDFLAGS:=-L$(LIBDIR) -l$(LIB) $(BOOSTLDFLAGS)

unittest: check

RUNARGS:=--report_level=detailed --log_level=all
#RUNARGS:=--report_level=short

check: $(TARGET)
	for trg in $+ ; do \
		echo === Testing $$trg... === ; \
		./$$trg $(RUNARGS) \
			|| exit 1 ; \
	done

%.x: %.cpp $(LIBDIR)/$(LIB).a
	$(CXX) -static -o $@ $< $(CXXFLAGS) $(LDFLAGS)

# if library is missing, build it
$(LIBDIR)/$(LIB).a:
	$(MAKE) -C $(LIBDIR)
