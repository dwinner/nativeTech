package main

import (
	"fmt"
	"time"
)

func printer(aReceiveChannel chan<- bool) {
	aReceiveChannel <- true
}

func writeToChannel(anRxChannel chan<- int, x int) {
	fmt.Println("1", x)
	anRxChannel <- x
	fmt.Println("2", x)
}

func f2(aReadChannel <-chan int, aWriteChannel chan<- int) {
	x := <-aReadChannel
	fmt.Println("Read (f2):", x)
	aWriteChannel <- x
}

func main() {
	intChan := make(chan int)
	go writeToChannel(intChan, 10)
	time.Sleep(1 * time.Second)
	fmt.Println("Read:", <-intChan)
	time.Sleep(1 * time.Second)
	close(intChan)

	chan1 := make(chan int, 1)
	chan2 := make(chan int, 1)

	// Write to channel
	chan1 <- 5
	f2(chan1, chan2)

	// Read from channel
	fmt.Println("Read (main):", <-chan2)
}
