package allocate

import (
   "bytes"
   "io"
)

func writeMessageBuffer(msg []byte, aByteBuf bytes.Buffer) {
   aByteBuf.Write(msg)
}

func writeMessageBufferPointer(msg []byte, aByteBuf *bytes.Buffer) {
   aByteBuf.Write(msg)
}

func writeMessageBufferWriter(msg []byte, aByteBuf io.Writer) {
   _, _ = aByteBuf.Write(msg)
}
