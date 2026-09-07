package allocate

import (
   "bytes"
   "testing"
)

func BenchmarkWBuf(aBenchCtx *testing.B) {
   msg := []byte("Mastering Go!")
   buffer := bytes.Buffer{}
   for i := 0; i < aBenchCtx.N; i++ {
      for range 50 {
         writeMessageBuffer(msg, buffer)
      }
   }
}

func BenchmarkWBufPointerNoReset(aBenchCtx *testing.B) {
   msg := []byte("Mastering Go!")
   buffer := new(bytes.Buffer)
   for i := 0; i < aBenchCtx.N; i++ {
      for range 50 {
         writeMessageBufferPointer(msg, buffer)
      }
   }
}

func BenchmarkWBufPointerReset(aBenchCtx *testing.B) {
   msg := []byte("Mastering Go!")
   buffer := new(bytes.Buffer)
   for i := 0; i < aBenchCtx.N; i++ {
      for range 50 {
         writeMessageBufferPointer(msg, buffer)
         buffer.Reset()
      }
   }
}

func BenchmarkWBufWriterReset(aBenchCtx *testing.B) {
   msg := []byte("Mastering Go!")
   buffer := new(bytes.Buffer)
   for i := 0; i < aBenchCtx.N; i++ {
      for range 50 {
         writeMessageBufferWriter(msg, buffer)
         buffer.Reset()
      }
   }
}
