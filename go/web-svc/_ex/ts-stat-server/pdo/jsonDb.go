package pdo

import (
   "os"
   "sync"
)

type JsonDb struct {
   jsonFile string
   guard    sync.Mutex
}

// Save Saves entries to json file
func (aJsonDb *JsonDb) Save(anEntryCollection *EntryCollection) error {
   file, err := os.Create(aJsonDb.jsonFile)
   if err != nil {
      return err
   }

   defer func(aFile *os.File) {
      _ = aFile.Close()
   }(file)

   aJsonDb.guard.Lock()
   defer aJsonDb.guard.Unlock()
   err = Serialize(&anEntryCollection.entries, file)
   if err != nil {
      return err
   }

   return nil
}

// Load Loads entries from json file
func (aJsonDb *JsonDb) Load(anEntryCollection *EntryCollection) error {
   _, err := os.Stat(aJsonDb.jsonFile)
   if err != nil {
      if os.IsNotExist(err) {
         return nil
      }

      return err
   }

   file, err := os.Open(aJsonDb.jsonFile)
   if err != nil {
      return err
   }

   defer func(aFile *os.File) {
      _ = aFile.Close()
   }(file)

   var entries = make([]Entry, 0)
   err = DeSerialize(&entries, file)
   if err != nil {
      return err
   }

   anEntryCollection.SetEntries(entries)

   return nil
}

func NewJsonDb(aJsonFile string) *JsonDb {
   return &JsonDb{
      jsonFile: aJsonFile,
      guard:    sync.Mutex{},
   }
}
