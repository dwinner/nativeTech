package main

import (
   "fmt"
)

func insert(anEntry *Entry) error {
   // If it already exists, do not add it
   _, ok := index[(*anEntry).Name]
   if ok {
      return fmt.Errorf("%s already exists", anEntry.Name)
   }

   phoneBook = append(phoneBook, *anEntry)

   // Update the index
   createIndex()
   err := saveJsonFile(Jsonfile)
   if err != nil {
      return err
   }

   return nil
}

func deleteEntry(aKey string) error {
   idx, ok := index[aKey]
   if !ok {
      return fmt.Errorf("%s cannot be found", aKey)
   }

   phoneBook = append(phoneBook[:idx], phoneBook[idx+1:]...)

   // Update the index - key does not exist anymore
   delete(index, aKey)
   err := saveJsonFile(Jsonfile)
   if err != nil {
      return err
   }

   return nil
}

func list() string {
   var all string
   for _, entry := range phoneBook {
      all = all + fmt.Sprintf("%s\t%d\t%f\t%f\n",
         entry.Name,
         entry.Len,
         entry.Mean,
         entry.StdDev)
   }

   return all
}

func search(aKey string) *Entry {
   idx, ok := index[aKey]
   if !ok {
      return nil
   }

   return &phoneBook[idx]
}
