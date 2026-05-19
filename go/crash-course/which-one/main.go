package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	arguments := os.Args
	if len(arguments) == 1 {
		fmt.Println("Please provide an argument")
		os.Exit(1)
	}

	file := arguments[1]
	path := os.Getenv("PATH")
	pathSplit := filepath.SplitList(path)

	allPaths := make([]string, 0)
	for _, directory := range pathSplit {
		fullPath := filepath.Join(directory, file)

		// Does it exist?
		fileInfo, err := os.Stat(fullPath)
		if err != nil {
			continue
		}

		mode := fileInfo.Mode()

		// Is it regular file?
		if !mode.IsRegular() {
			continue
		}

		allPaths = append(allPaths, fullPath)
	}

	for _, path := range allPaths {
		fmt.Println(path)
	}
}
