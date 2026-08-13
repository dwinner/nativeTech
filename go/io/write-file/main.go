package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

func main() {
	buffer := []byte("Data to write\n")
	fileF1, err := os.Create("f1.txt")
	if err != nil {
		fmt.Println("Cannot create file", err)
		return
	}

	defer safelyClose(fileF1)

	_, err = fmt.Fprintf(fileF1, "%s", string(buffer))
	if err != nil {
		return
	}

	fileF2, err := os.Create("f2.txt")
	if err != nil {
		fmt.Println("Cannot create file", err)
		return
	}

	defer safelyClose(fileF2)

	writtenLen, err := fileF2.WriteString(string(buffer))
	fmt.Printf("wrote %d bytes\n", writtenLen)

	fileF3, err := os.Create("f3.txt")
	if err != nil {
		fmt.Println(err)
		return
	}

	bufWriter := bufio.NewWriter(fileF3)
	writtenLen, err = bufWriter.WriteString(string(buffer))
	fmt.Printf("wrote %d bytes\n", writtenLen)
	flushErr := bufWriter.Flush()
	if flushErr != nil {
		return
	}

	f4FileName := "f4.txt"
	fileF4, err := os.Create(f4FileName)
	if err != nil {
		fmt.Println(err)
		return
	}

	defer safelyClose(fileF4)

	for range 5 {
		writtenLen, err = io.WriteString(fileF4, string(buffer))
		if err != nil {
			fmt.Println(err)
			return
		}

		fmt.Printf("wrote %d bytes\n", writtenLen)
	}

	// Append to a file
	fileF4, err = os.OpenFile(f4FileName, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println(err)
		return
	}

	defer safelyClose(fileF4)

	// Write() needs a byte slice
	writtenLen, err = fileF4.Write([]byte("Put some more data at the end.\n"))
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("wrote %d bytes\n", writtenLen)
}

func safelyClose(aFile *os.File) {
	err := aFile.Close()
	if err != nil {
		fmt.Println(err)
		os.Exit(-1)
	}
}
