package main

import (
   "encoding/json"
   "fmt"
   "io"
   "math"
   "net/http"
   "os"
   "slices"
   "time"
)

type Entry struct {
   Name    string
   Len     int
   Minimum float64
   Maximum float64
   Mean    float64
   StdDev  float64
}

func process(aFilename string, values []float64) Entry {
   currentEntry := Entry{}
   currentEntry.Name = aFilename
   currentEntry.Len = len(values)
   currentEntry.Minimum = slices.Min(values)
   currentEntry.Maximum = slices.Max(values)
   meanValue, standardDeviation := stdDev(values)
   currentEntry.Mean = meanValue
   currentEntry.StdDev = standardDeviation

   return currentEntry
}

func stdDev(aFloatSlice []float64) (float64, float64) {
   sum := float64(0)
   for _, val := range aFloatSlice {
      sum = sum + val
   }

   meanValue := sum / float64(len(aFloatSlice))

   // Standard deviation
   var squared float64
   for idx := range aFloatSlice {
      squared = squared + math.Pow(aFloatSlice[idx]-meanValue, 2)
   }

   standardDeviation := math.Sqrt(squared / float64(len(aFloatSlice)))
   return meanValue, standardDeviation
}

// Jsonfile resides in the current directory
var Jsonfile = "./data.json"

type PhoneBook []Entry

var data = PhoneBook{}
var index map[string]int

// DeSerialize decodes a serialized slice with JSON records
func DeSerialize(aSlice any, aReader io.Reader) error {
   decoder := json.NewDecoder(aReader)
   return decoder.Decode(aSlice)
}

// Serialize serializes a slice with JSON records
func Serialize(aSlice any, aWriter io.Writer) error {
   encoder := json.NewEncoder(aWriter)
   return encoder.Encode(aSlice)
}

func saveJsonFile(aFilepath string) error {
   file, err := os.Create(aFilepath)
   if err != nil {
      return err
   }

   defer func(file *os.File) {
      _ = file.Close()
   }(file)

   err = Serialize(&data, file)
   if err != nil {
      return err
   }

   return nil
}

func readJsonFile(aFilepath string) error {
   _, err := os.Stat(aFilepath)
   if err != nil {
      if os.IsNotExist(err) {
         //_, _ = os.OpenFile(aFilepath, os.O_RDONLY|os.O_CREATE, 0644)
         return nil
      }

      return err
   }

   file, err := os.Open(aFilepath)
   if err != nil {
      return err
   }

   defer func(file *os.File) {
      _ = file.Close()
   }(file)

   err = DeSerialize(&data, file)
   if err != nil {
      return err
   }

   return nil
}

func createIndex() {
   index = make(map[string]int)
   for idx, entry := range data {
      key := entry.Name
      index[key] = idx
   }
}

func insert(anEntry *Entry) error {
   // If it already exists, do not add it
   _, ok := index[(*anEntry).Name]
   if ok {
      return fmt.Errorf("%s already exists", anEntry.Name)
   }

   data = append(data, *anEntry)

   // Update the index
   createIndex()
   err := saveJsonFile(Jsonfile)
   if err != nil {
      return err
   }

   return nil
}

func deleteEntry(aKey string) error {
   idx, ok := index[aKey]
   if !ok {
      return fmt.Errorf("%s cannot be found", aKey)
   }

   data = append(data[:idx], data[idx+1:]...)

   // Update the index - key does not exist anymore
   delete(index, aKey)
   err := saveJsonFile(Jsonfile)
   if err != nil {
      return err
   }

   return nil
}

func search(aKey string) *Entry {
   idx, ok := index[aKey]
   if !ok {
      return nil
   }

   return &data[idx]
}

func list() string {
   var all string
   for _, entry := range data {
      all = all + fmt.Sprintf("%s\t%d\t%f\t%f\n",
         entry.Name,
         entry.Len,
         entry.Mean,
         entry.StdDev)
   }

   return all
}

func main() {
   err := readJsonFile(Jsonfile)
   if err != nil && err != io.EOF {
      fmt.Println("Error:", err)
      return
   }

   createIndex()
   serveMux := http.NewServeMux()
   httpServer := &http.Server{
      Addr:         PORT,
      Handler:      serveMux,
      IdleTimeout:  10 * time.Second,
      ReadTimeout:  time.Second,
      WriteTimeout: time.Second,
   }

   serveMux.Handle("/list", http.HandlerFunc(listHandler))
   serveMux.Handle("/insert/", http.HandlerFunc(insertHandler))
   serveMux.Handle("/insert", http.HandlerFunc(insertHandler))
   serveMux.Handle("/search", http.HandlerFunc(searchHandler))
   serveMux.Handle("/search/", http.HandlerFunc(searchHandler))
   serveMux.Handle("/delete/", http.HandlerFunc(deleteHandler))
   serveMux.Handle("/status", http.HandlerFunc(statusHandler))
   serveMux.Handle("/", http.HandlerFunc(defaultHandler))

   fmt.Println("Ready to serve at", PORT)
   err = httpServer.ListenAndServe()
   if err != nil {
      fmt.Println(err)
      return
   }
}
