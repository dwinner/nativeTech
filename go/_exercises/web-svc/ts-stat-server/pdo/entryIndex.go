package pdo

import "sync"

type EntryIndex struct {
   index   map[string]int
   rwGuard sync.RWMutex
}

func (anEntryIndex *EntryIndex) Delete(aKey string) {
   anEntryIndex.rwGuard.Lock()
   defer anEntryIndex.rwGuard.Unlock()
   delete(anEntryIndex.index, aKey)
}

func (anEntryIndex *EntryIndex) GetIndex() map[string]int {
   anEntryIndex.rwGuard.RLock()
   defer anEntryIndex.rwGuard.RUnlock()
   return anEntryIndex.index
}

func (anEntryIndex *EntryIndex) CreateIndex(anEntryCollection *EntryCollection) {
   entries := anEntryCollection.GetEntries()
   for index, entry := range entries {
      key := entry.Name
      anEntryIndex.rwGuard.Lock()
      anEntryIndex.index[key] = index
      anEntryIndex.rwGuard.Unlock()
   }
}

func NewEntryIndex() *EntryIndex {
   indexMap := make(map[string]int)
   return &EntryIndex{
      index:   indexMap,
      rwGuard: sync.RWMutex{},
   }
}
