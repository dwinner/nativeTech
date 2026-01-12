
// https://godbolt.org/z/rn5oK3M6e 
void testDoubleIt() {
    assertTrue( doubleIt(0) == 0 );
    assertTrue( doubleIt(-1) == -2 );
    assertTrue( doubleIt(1) == 2 );
    assertTrue( doubleIt(5) == 10 );
}