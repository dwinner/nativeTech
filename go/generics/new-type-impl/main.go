package main

import (
   "errors"
   "fmt"
)

type TreeLast[T any] []T

func (aTree TreeLast[T]) replaceLast(element T) (TreeLast[T], error) {
   if len(aTree) == 0 {
      return aTree, errors.New("this is empty")
   }

   aTree[len(aTree)-1] = element
   return aTree, nil
}

func main() {
   tempStr := TreeLast[string]{"aa", "bb"}
   fmt.Println(tempStr)
   tempStr.replaceLast("cc")
   fmt.Println(tempStr)

   tempInt := TreeLast[int]{12, -3}
   fmt.Println(tempInt)
   tempInt.replaceLast(0)
   fmt.Println(tempInt)
}
