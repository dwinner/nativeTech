all: check

# groups

check: runtest unittest

# decl
LIBDIR:=src/lib/
PROGDIR:=src/prog/
TESTDIR:=test/


# build

library:
	$(MAKE) -C $(LIBDIR)

program: library
	$(MAKE) -C $(PROGDIR)

# tests

unittest: library
	$(MAKE) -C $(TESTDIR) unittest

runtest: program
	$(MAKE) -C $(PROGDIR) check

# maintenance
clean:
	$(MAKE) -C $(LIBDIR) clean
	$(MAKE) -C $(TESTDIR) clean
	$(MAKE) -C $(PROGDIR) clean

