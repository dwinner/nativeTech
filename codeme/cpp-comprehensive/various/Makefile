# Makefile

########################
# Defs

TARGETS=\
	02Modern101.x \
	02Modern102.x \
	04-grundelemente-01.x \
	04floats.x \
	29async101.x \
	41vector-noncopy.x \
	41moveTiming.x \
	41reversemap.x \
	41set-vector.x \
	41allocator.x \
	userlitVartemplate.x \
	foldExpr.x \
	07double-prec.x \
	45cpp17variant.x \
	45cpp17any.x \
	45cpp17sample.x \
	28chronoExceptions.x \
	28chronoTzdb.x \
	24containerPerf.x \
	24insertionTimes.x \
	22capis.x \


#not yet:
#	24ranges4.x \


#############
# source suffixes:
# - %.11.cpp : C++11
# - %.11.cppp : C++11 w/formatting
# - %.cxx : C++14
# - %.cxxx : C++14  w/formatting
# - %.CPP : C++17/C++1z
# - %.20.cpp : C++20/C++2a
# - %.23.cpp : C++23/C++2b
# - %.c : C

########################
# Env

#CXX:=../bin/docker-compile.sh
#CXX:=docker run --rm -i -t --volume $(CURDIR):/workdir --workdir /workdir gcc:13.2 g++ -fanalyzer -ltbb
CXX:=docker run --rm --volume $(CURDIR):/workdir --workdir /workdir gcc:14 g++
EXEC:=../bin/docker-compile.sh

#CXXLIBS=-lz
CXXLIBS=

CXXFLAGS=-Wall -Wextra -std=c++11 -lpthread -pthread -O0 -Wno-missing-field-initializers -g
CXX14FLAGS=-Wall -Wextra -std=c++1y -lpthread -pthread -O0 -Wno-missing-field-initializers -g
CXX17FLAGS=-Wall -Wextra -std=c++17 -lpthread -pthread -O0 -Wno-missing-field-initializers -g
CXX20FLAGS=-Wall -Wextra -std=c++20 -lpthread -pthread -Wno-missing-field-initializers -g
CXX23FLAGS=-Wall -Wextra -std=c++23 -lpthread -pthread -lz -Wno-missing-field-initializers -g -fext-numeric-literals -O2 -fno-diagnostics-color


########################
# Rules

all: $(TARGETS)

%.11.cpp: %.11.cppp
	./cc1plus < $< > $@

%.cxx: %.cxxx
	./cc1plus < $< > $@

%.x: %.11.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< $(CXXLIBS)

%.x: %.cxx
	$(CXX) $(CXX14FLAGS) -o $@ $<

%.x: %.CPP
	$(CXX) $(CXX17FLAGS) -o $@ $<

%.x: %.20.cpp
	$(CXX) $(CXX20FLAGS) -o $@ $<

%.x: %.23.cpp
	$(CXX) $(CXX23FLAGS) -o $@ $<

%.x: %.c
	$(CC) $(CFLAGS) -o $@ $< $(CLIBS)

%.x.txt: %.x
	$(EXEC) ./$< | tee $@

46-zpipe.x: 46-zpipe.c
	gcc 46-zpipe.c -o 46-zpipe.x -lz


########################
# Aux

clean:
	rm -f *.o check-report.txt
	rm -f $(TARGETS) *.x *.x.txt *.should_have_failed

mrproper: clean
