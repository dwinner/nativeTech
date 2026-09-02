package intRE

import (
   "math/rand"
   "strconv"
   "testing"
)

func random(min, max int) int {
   return rand.Intn(max-min) + min
}

func TestMatchInt(aTestCtx *testing.T) {
   if matchInt("") {
      aTestCtx.Error(`matchInt("") != false`)
   }

   if matchInt("00") == false {
      aTestCtx.Error(`matchInt("00") != true`)
   }

   if matchInt("-00") == false {
      aTestCtx.Error(`matchInt("-00") != true`)
   }

   if matchInt("+00") == false {
      aTestCtx.Error(`matchInt("+00") != true`)
   }
}

func TestWithRandom(aTestCtx *testing.T) {
   rndStr := strconv.Itoa(random(-100000, 19999))
   if matchInt(rndStr) == false {
      aTestCtx.Error("n = ", rndStr)
   }
}
