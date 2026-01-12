# maintenance

# g++, g++-6 -- select compiler and its version
# -std=c++14, -std=c++1y, -std=c++17 -- select c++-standard
# -DTDEBUG -- compile my own debug and trace code
#CXX:=g++ -std=c++14

clean:
	$(RM) *.a *.o *.x
