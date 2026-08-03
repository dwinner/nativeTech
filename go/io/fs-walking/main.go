package main

import (
   "embed"
   "fmt"
   "io/fs"
   "os"
)

//go:embed static
var embeddedDir embed.FS
var searchString string

func walkFunction(aPath string, aDirEntry fs.DirEntry, anError error) error {
   if anError != nil {
      return anError
   }

   fmt.Printf("Path=%q, isDir=%v\n", aPath, aDirEntry.IsDir())
   return nil
}

func walkSearch(aPath string, aDirEntry fs.DirEntry, anError error) error {
   if anError != nil {
      return anError
   }

   if aDirEntry.Name() == searchString {
      fileInfo, err := fs.Stat(embeddedDir, aPath)
      if err != nil {
         return err
      }

      fmt.Println("Found", aPath, "with size", fileInfo.Size())
      return nil
   }

   return nil
}

func list(aFile embed.FS) error {
   return fs.WalkDir(aFile, ".", walkFunction)
}

func search(aFile embed.FS) error {
   return fs.WalkDir(aFile, ".", walkSearch)
}

func extract(aFile embed.FS, aFilepath string) ([]byte, error) {
   bytesRead, err := fs.ReadFile(aFile, aFilepath)
   if err != nil {
      return nil, err
   }

   return bytesRead, nil
}

func writeToFile(bytesToWrite []byte, aPath string) error {
   fd, err := os.OpenFile(aPath, os.O_CREATE|os.O_WRONLY, 0644)
   if err != nil {
      return err
   }

   defer fd.Close()
   writtenLen, err := fd.Write(bytesToWrite)
   if err != nil {
      return err
   }

   fmt.Printf("wrote %d bytes\n", writtenLen)
   return nil
}

func main() {
   // At this point we do not know what is included in ./static

   // List all files
   err := list(embeddedDir)
   if err != nil {
      fmt.Println(err)
      return
   }

   // Search
   searchString = "file.txt"
   err = search(embeddedDir)
   if err != nil {
      fmt.Println(err)
      return
   }

   // Extract into a byte slice
   buffer, err := extract(embeddedDir, "static/file.txt")
   if err != nil {
      fmt.Println(err)
      return
   }

   // Save it to an actual file
   err = writeToFile(buffer, "/tmp/IOFS.txt")
   if err != nil {
      fmt.Println(err)
      return
   }
}
