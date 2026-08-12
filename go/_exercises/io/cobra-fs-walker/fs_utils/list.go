package fs_utils

import (
   "fmt"
   "io/fs"
)

func List(aFileSystem fs.FS, aRootDir string) error {
   return fs.WalkDir(aFileSystem, aRootDir, walkFn)
}

func walkFn(aPath string, aDirEntry fs.DirEntry, anError error) error {
   if anError != nil {
      return anError
   }

   fmt.Printf("Path=%q, isDir=%v\n", aPath, aDirEntry.IsDir())

   return nil
}
