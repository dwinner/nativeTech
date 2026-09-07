package main

import (
   "testing"
)

func TestAddInt(aTestCtx *testing.T) {
   testCases := []struct {
      x, y, want int
   }{
      {1, 2, 3},
      {1, 0, 1},
      {100, 10, 110},
   }

   for _, testCase := range testCases {
      result := AddInt(testCase.x, testCase.y)
      if result != testCase.want {
         aTestCtx.Errorf("X: %d, Y: %d, want %d", testCase.x, testCase.y, testCase.want)
      }
   }
}

func FuzzAddInt(aFuzzCtx *testing.F) {
   testCases := []struct {
      x, y int
   }{
      {0, 1},
      {0, 100},
   }

   for _, testCase := range testCases {
      aFuzzCtx.Add(testCase.x, testCase.y)
   }

   aFuzzCtx.Fuzz(func(testing *testing.T, x, y int) {
      result := AddInt(x, y)
      if result != x+y {
         testing.Errorf("X: %d, Y: %d, Result %d, want %d", x, y, result, x+y)
      }
   })
}
