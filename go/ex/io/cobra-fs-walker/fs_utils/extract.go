package fs_utils

import "io/fs"

func Extract(aFileSystem fs.FS, aFilePath string) ([]byte, error) {
   bytesRead, err := fs.ReadFile(aFileSystem, aFilePath)
   if err != nil {
      return nil, err
   }

   return bytesRead, nil
}
