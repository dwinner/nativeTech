package fib_test

func fib(aNumber uint64) uint64 {
   if aNumber <= 1 {
      return aNumber
   }

   num1, num2 := uint64(0), uint64(1)
   for i := uint64(2); i <= aNumber; i++ {
      num1, num2 = num2, num1+num2
   }

   return num2
}
