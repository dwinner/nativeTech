package quickt

import (
   "testing"
   "testing/quick"
)

var N = 1000000

func TestWithItself(aTestCtx *testing.T) {
   condition := func(a, b Point2D) bool {
      return Add(a, b) == Add(b, a)
   }

   err := quick.Check(condition, &quick.Config{MaxCount: N})
   if err != nil {
      aTestCtx.Errorf("Error: %v", err)
   }
}

func TestThree(aTestCtx *testing.T) {
   condition := func(a, b, c Point2D) bool {
      return Add(Add(a, b), c) == Add(a, b)
   }

   err := quick.Check(condition, &quick.Config{MaxCount: N})
   if err != nil {
      aTestCtx.Errorf("Error: %v", err)
   }
}
