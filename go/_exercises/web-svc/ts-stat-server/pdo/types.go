package pdo

/* TODO: Sync. access to Jsonfile, phoneBook, index */

type Entry struct {
   Name    string
   Len     int
   Minimum float64
   Maximum float64
   Mean    float64
   StdDev  float64
}

// Jsonfile resides in the current directory
var Jsonfile = "./data.json"

type PhoneBook []Entry

var phoneBook = PhoneBook{}
var index map[string]int
