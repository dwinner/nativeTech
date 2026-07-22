package main

import (
   "fmt"
   "slices"
)

func main() {
   slice1 := []int{1, 2, -1, -2}
   slice2 := slices.Clone(slice1)
   slice3 := slices.Clone(slice1[2:])
   fmt.Println(slice1[2], slice2[2], slice3[0])
   slice1[2] = 0
   slice1[3] = 0
   fmt.Println(slice1[2], slice2[2], slice3[0])

   slice1 = slices.Compact(slice1)
   fmt.Println("s1 (compact):", slice1)
   fmt.Println(slices.Contains(slice1, 2), slices.Contains(slice1, -2))

   slice4 := make([]int, 10, 100)
   fmt.Println("Len:", len(slice4), "Cap:", cap(slice4))
   slice4 = slices.Clip(slice4)
   fmt.Println("Len:", len(slice4), "Cap:", cap(slice4))

   fmt.Println("Mix:", slices.Min(slice1), "Max:", slices.Max(slice1))
   // Replace s2[1] and s2[2]
   slice2 = slices.Replace(slice2, 1, 3, 100, 200)
   fmt.Println("s2 (replaced):", slice2)
   slices.Sort(slice2)
   fmt.Println("s2 (sorted):", slice2)
}
