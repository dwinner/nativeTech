package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

type atomCounter struct {
	val atomic.Int64
}

func (anAtomCounter *atomCounter) Value() int64 {
	return anAtomCounter.val.Load()
}

func main() {
	X := 100
	Y := 4
	var waitGroup sync.WaitGroup
	counter := atomCounter{}
	for range X {
		waitGroup.Go(func() {
			for range Y {
				counter.val.Add(1)
			}
		})
	}

	waitGroup.Wait()
	fmt.Println(counter.Value())
}
