package cleanup

import (
   "fmt"
   "os"
   "path"
   "testing"
)

func myCleanUp() func() {
   return func() {
      fmt.Println("Cleaning up!")
   }
}

func TestFoo(aTesting *testing.T) {
   t1 := path.Join(os.TempDir(), "test01")
   t2 := path.Join(os.TempDir(), "test02")
   err := os.Mkdir(t1, 0755)
   if err != nil {
      aTesting.Error("os.Mkdir() failed:", err)
      return
   }

   defer aTesting.Cleanup(func() {
      err = os.Remove(t1)
      if err != nil {
         aTesting.Error("os.Mkdir() failed:", err)
      }
   })

   err = os.Mkdir(t2, 0755)
   if err != nil {
      aTesting.Error("os.Mkdir() failed:", err)
      return
   }
}

func TestBar(aTesting *testing.T) {
   t1 := aTesting.TempDir()
   fmt.Println(t1)
   aTesting.Cleanup(myCleanUp())
}
