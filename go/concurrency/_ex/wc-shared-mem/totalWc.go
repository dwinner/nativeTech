package main

import (
   "fmt"
   "sync"
)

type TotalWc struct {
   CharCount   int64
   LineCount   int64
   WordCount   int64
   UpdateGuard *sync.Mutex
}

func (aTotalWc *TotalWc) Print() {
   fmt.Printf("Count of chars : '%d'\n", aTotalWc.CharCount)
   fmt.Printf("Count of lines : '%d'\n", aTotalWc.LineCount)
   fmt.Printf("Count of words : '%d'\n", aTotalWc.WordCount)
}

func NewTotalWc(charCount int64, lineCount int64, wordCount int64, updateGuard *sync.Mutex) *TotalWc {
   return &TotalWc{
      CharCount:   charCount,
      LineCount:   lineCount,
      WordCount:   wordCount,
      UpdateGuard: updateGuard}
}

func (aTotalWc *TotalWc) UpdateCounters(aCharCount int64, aLineCount int64, aWordCount int64) {
   aTotalWc.UpdateGuard.Lock()
   defer aTotalWc.UpdateGuard.Unlock()
   aTotalWc.CharCount += aCharCount
   aTotalWc.LineCount += aLineCount
   aTotalWc.WordCount += aWordCount
}
