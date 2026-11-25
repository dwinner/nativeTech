package fs_utils

import (
   "fmt"
   "io/fs"
)

var searchString string
var rootFileSystem fs.FS

func Search(aFileSystem fs.FS, aRootDir string, aSearchString string) error {
   searchString = aSearchString
   rootFileSystem = aFileSystem
   return fs.WalkDir(aFileSystem, aRootDir, searchFn)
}

func searchFn(aPath string, aDirEntry fs.DirEntry, anError error) error {
   if anError != nil {
      return anError
   }

   if aDirEntry.Name() == searchString {
      fileInfo, anError := fs.Stat(rootFileSystem, aPath)
      if anError != nil {
         return anError
      }

      fmt.Println("Found '", aPath, "' with size ", fileInfo.Size())
      return nil
   }

   return nil
}
