package main

import (
   "fmt"
   "os"
   "strconv"
)

type Ar2x2 [2][2]int

// Add - Traditional Add() function
func Add(matrix1, matrix2 Ar2x2) Ar2x2 {
   matrix := Ar2x2{}
   for i := 0; i < 2; i++ {
      for j := 0; j < 2; j++ {
         matrix[i][j] = matrix1[i][j] + matrix2[i][j]
      }
   }

   return matrix
}

// Add - Type method Add()
func (aMatrix *Ar2x2) Add(matrix Ar2x2) {
   for i := 0; i < 2; i++ {
      for j := 0; j < 2; j++ {
         aMatrix[i][j] = aMatrix[i][j] + matrix[i][j]
      }
   }
}

// Subtract - Type method Subtract()
func (aMatrix *Ar2x2) Subtract(matrix Ar2x2) {
   for i := 0; i < 2; i++ {
      for j := 0; j < 2; j++ {
         aMatrix[i][j] = aMatrix[i][j] - matrix[i][j]
      }
   }
}

// Multiply - Type method Multiply()
func (aMatrix *Ar2x2) Multiply(matrix Ar2x2) {
   aMatrix[0][0] = aMatrix[0][0]*matrix[0][0] + aMatrix[0][1]*matrix[1][0]
   aMatrix[1][0] = aMatrix[1][0]*matrix[0][0] + aMatrix[1][1]*matrix[1][0]
   aMatrix[0][1] = aMatrix[0][0]*matrix[0][1] + aMatrix[0][1]*matrix[1][1]
   aMatrix[1][1] = aMatrix[1][0]*matrix[0][1] + aMatrix[1][1]*matrix[1][1]
}

func main() {
   if len(os.Args) != 9 {
      fmt.Println("Need 8 integers")
      return
   }

   matrix := [8]int{}
   for index, value := range os.Args[1:] {
      input, err := strconv.Atoi(value)
      if err != nil {
         fmt.Println(err)
         return
      }

      matrix[index] = input
   }

   mtx1 := Ar2x2{{matrix[0], matrix[1]}, {matrix[2], matrix[3]}}
   mtx2 := Ar2x2{{matrix[4], matrix[5]}, {matrix[6], matrix[7]}}

   fmt.Println("Traditional a+b:", Add(mtx1, mtx2))
   mtx1.Add(mtx2)
   fmt.Println("a+b:", mtx1)
   mtx1.Subtract(mtx1)
   fmt.Println("a-a:", mtx1)

   mtx1 = Ar2x2{{matrix[0], matrix[1]}, {matrix[2], matrix[3]}}
   mtx1.Multiply(mtx2)
   fmt.Println("a*b:", mtx1)

   mtx1 = Ar2x2{{matrix[0], matrix[1]}, {matrix[2], matrix[3]}}
   mtx2.Multiply(mtx1)
   fmt.Println("b*a:", mtx2)
}
