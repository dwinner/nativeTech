package main

import (
   "fmt"
   "os"
   "path/filepath"
)

func GetSize(aPath string) (int64, error) {
   contents, err := os.ReadDir(aPath)
   if err != nil {
      return -1, err
   }

   var total int64
   for _, entry := range contents {
      if entry.IsDir() {
         // Visit directory entries
         temp, err := GetSize(filepath.Join(aPath, entry.Name()))
         if err != nil {
            return -1, err
         }

         total += temp
      } else {
         // Get size of each non-directory entry
         info, err := entry.Info()
         if err != nil {
            return -1, err
         }

         // Returns an int64 value
         total += info.Size()
      }
   }

   return total, nil
}

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Need a <Directory>")
      return
   }

   root, err := filepath.EvalSymlinks(arguments[1])
   fileInfo, err := os.Stat(root)
   if err != nil {
      fmt.Println(err)
      return
   }

   fileInfo, _ = os.Lstat(root)
   mode := fileInfo.Mode()
   if !mode.IsDir() {
      fmt.Println(root, "not a directory!")
      return
   }

   size, err := GetSize(root)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println("Total Size:", size)
}
