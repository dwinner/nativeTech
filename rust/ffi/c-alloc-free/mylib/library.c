#include "library.h"

int* create_int_array(size_t size)
{
   int* arr = malloc(size * sizeof(int));
   if (arr == NULL)
   {
      return NULL;
   }

   for (size_t i = 0; i < size; i++)
   {
      arr[i] = i;
   }

   return arr;
}

void free_int_array(int* arr)
{
   free(arr);
}
