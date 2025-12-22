# Makefile

########################
# Defs

TARGETS=\
	15-fib_test01.x \
	15-fib_test02.x \


########################
# Env

#INSTPATH=/home/towi/src/gcc/inst
#INSTPATH=/usr
#CXX:=$(INSTPATH)/bin/g++
CXXFLAGS=-Wall -Wextra -std=c++11 -lpthread -pthread -O0 -Wno-missing-field-initializers
CXX14FLAGS=-Wall -Wextra -std=c++1y -lpthread -pthread -O0 -Wno-missing-field-initializers
CXXLIBS=-lz

########################
# Rules

all: $(TARGETS)

run: $(TARGETS) demo-15
	for x in $(TARGETS) ; do \
		echo === Executing: ./$$x === ;\
		./$$x ;\
	done

%.cpp: %.cppp
	./cc1plus < $< > $@

%.x: %.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< $(CXXLIBS)

%.x: %.cxx
	$(CXX) $(CXX14FLAGS) -o $@ $<

%.x: %.c
	$(CC) $(CFLAGS) -o $@ $< $(CLIBS)

15-fib_test01.x: 15-fib.cpp 15-fib_test01.cpp
	g++ $^ -o $@ $(CXXLIBS)
demo-15: 15-fib_test01.x
	./15-fib_test01.x -r detailed -p

15-fib_test02.x: 15-fib.cpp 15-fib_test02.cpp
	g++ -static $^ -o $@ $(CXXLIBS) -lboost_unit_test_framework

########################
# Aux

clean:
	rm -f *.o
	rm -f $(TARGETS) *.x

mrproper: clean
