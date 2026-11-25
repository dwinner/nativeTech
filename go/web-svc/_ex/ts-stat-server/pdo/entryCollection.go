package pdo

import "sync"

type EntryCollection struct {
   entries []Entry
   rwGuard sync.RWMutex
}

func (anEntryCollection *EntryCollection) Add(anEntry *Entry) {
   anEntryCollection.rwGuard.Lock()
   defer anEntryCollection.rwGuard.Unlock()
   anEntryCollection.entries = append(anEntryCollection.entries, *anEntry)
}

func (anEntryCollection *EntryCollection) Len() int {
   anEntryCollection.rwGuard.RLock()
   defer anEntryCollection.rwGuard.RUnlock()
   return len(anEntryCollection.entries)
}

func (anEntryCollection *EntryCollection) GetEntries() []Entry {
   anEntryCollection.rwGuard.RLock()
   defer anEntryCollection.rwGuard.RUnlock()
   return anEntryCollection.entries
}

func (anEntryCollection *EntryCollection) SetEntries(entries []Entry) {
   anEntryCollection.rwGuard.Lock()
   defer anEntryCollection.rwGuard.Unlock()
   anEntryCollection.entries = entries
}

func NewEntryCollection() *EntryCollection {
   entries := make([]Entry, 0)
   return &EntryCollection{
      entries,
      sync.RWMutex{},
   }
}
