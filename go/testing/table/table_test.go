package division

import (
   "testing"
)

type myTest struct {
   a             int
   b             int
   expectedInt   int
   expectedFloat float64
}

var tests = []myTest{
   {a: 1, b: 2, expectedInt: 0, expectedFloat: 0.5},
   {a: 5, b: 10, expectedInt: 0, expectedFloat: 0.5},
   {a: 2, b: 2, expectedInt: 1, expectedFloat: 1.0},
   {a: 4, b: 2, expectedInt: 2, expectedFloat: 2.0},
   {a: 5, b: 2, expectedInt: 2, expectedFloat: 2.5},
   {a: 5, b: 4, expectedInt: 1, expectedFloat: 1.2},
}

func TestAll(aTestCtx *testing.T) {
   aTestCtx.Parallel()

   for _, test := range tests {
      actualInt := intDiv(test.a, test.b)
      if actualInt != test.expectedInt {
         aTestCtx.Errorf("Expected %d, got %d", test.expectedInt, actualInt)
      }

      actualFloat := floatDiv(test.a, test.b)
      if actualFloat != test.expectedFloat {
         aTestCtx.Errorf("Expected %f, got %f", test.expectedFloat, actualFloat)
      }
   }
}
