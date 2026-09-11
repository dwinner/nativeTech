package allocate

import (
   "testing"
)

func BenchmarkWrite(aBenchCtx *testing.B) {
   msg := []byte("Mastering Go!")
   for i := 0; i < aBenchCtx.N; i++ {
      for range 50 {
         writeMessage(msg)
      }
   }
}
