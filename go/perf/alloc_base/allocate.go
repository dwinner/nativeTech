package allocate

import (
   "bytes"
)

func writeMessage(msg []byte) {
   newBytes := new(bytes.Buffer)
   newBytes.Write(msg)
}
