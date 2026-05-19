#include "library.h"

#ifdef __cplusplus
extern "C"
{

#endif

unsigned long long factorial(unsigned int n)
{
   if (n == 0)
   {
      return 1;
   }

   return n * factorial(n - 1);
}

#ifdef __cplusplus
}
#endif
