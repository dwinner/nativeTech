package main

import (
   "fmt"
   "pdo"
)

func insert(anEntry *pdo.Entry) error {
   // If it already exists, do not add it
   index := entryIndex.GetIndex()
   _, ok := index[(*anEntry).Name]
   if ok {
      return fmt.Errorf("%s already exists", anEntry.Name)
   }

   entryCollection.Add(anEntry)

   // Update the index
   entryIndex.CreateIndex(entryCollection)
   err := jsonDb.Save(entryCollection)
   if err != nil {
      return err
   }

   return nil
}

func deleteByKey(aKey string) error {
   index := entryIndex.GetIndex()
   idx, ok := index[aKey]
   if !ok {
      return fmt.Errorf("%s cannot be found", aKey)
   }

   entries := entryCollection.GetEntries()
   entries = append(entries[:idx], entries[idx+1:]...)
   entryCollection.SetEntries(entries)

   // Update the index - key does not exist anymore
   entryIndex.Delete(aKey)
   err := jsonDb.Save(entryCollection)
   if err != nil {
      return err
   }

   return nil
}

func list() string {
   var all string
   entries := entryCollection.GetEntries()
   for _, entry := range entries {
      all = all + fmt.Sprintf("%s\t%d\t%f\t%f\n",
         entry.Name,
         entry.Len,
         entry.Mean,
         entry.StdDev)
   }

   return all
}

func search(aKey string) *pdo.Entry {
   index := entryIndex.GetIndex()
   idx, ok := index[aKey]
   if !ok {
      return nil
   }

   entries := entryCollection.GetEntries()
   return &entries[idx]
}
