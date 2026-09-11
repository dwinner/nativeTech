package server

import (
   "testing"
)

func TestMap(aTestCtx *testing.T) {
   key := "server"
   server, ok := Data[key]
   if !ok {
      aTestCtx.Fatalf("Key %s not found!", key)
   }

   key = "port"
   port, ok := Data[key]
   if !ok {
      aTestCtx.Fatalf("Key %s not found!", key)
   }

   aTestCtx.Log("Connecting to", server, "@port", port)
}
