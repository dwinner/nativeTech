
// https://godbolt.org/z/sTaoeqfxr 
void testDoubleIt5() {
    // arrange
    auto param = 5;
    // act
    auto result = doubleIt(param);
    // assert
    assertTrue(result == 10);
}