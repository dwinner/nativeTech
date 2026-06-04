package main

import (
   "encoding/csv"
   "log"
   "os"
)

type Record struct {
   Name       string
   Surname    string
   Number     string
   LastAccess string
}

var myData []Record

func readCsvFile(aFilepath string) ([][]string, error) {
   _, err := os.Stat(aFilepath)
   if err != nil {
      return nil, err
   }

   file, err := os.Open(aFilepath)
   if err != nil {
      return nil, err
   }

   defer func(file *os.File) {
      err := file.Close()
      if err != nil {
         log.Fatal(err)
      }
   }(file)

   // CSV file read all at once
   // lines is a [][]string variable
   lines, err := csv.NewReader(file).ReadAll()
   if err != nil {
      return [][]string{}, err
   }

   return lines, nil
}

func saveCsvFile(filepath string) error {
   csvFile, err := os.Create(filepath)
   if err != nil {
      return err
   }

   defer func(csvFile *os.File) {
      err := csvFile.Close()
      if err != nil {
         log.Fatal(err)
      }
   }(csvFile)

   csvWriter := csv.NewWriter(csvFile)

   // Changing the default field delimiter to tab
   csvWriter.Comma = '\t'
   for _, row := range myData {
      temp := []string{row.Name, row.Surname, row.Number, row.LastAccess}
      err = csvWriter.Write(temp)
      if err != nil {
         return err
      }
   }

   csvWriter.Flush()
   return nil
}

func main() {
   if len(os.Args) != 3 {
      log.Println("csvData input output!")
      os.Exit(1)
   }

   input := os.Args[1]
   output := os.Args[2]
   lines, err := readCsvFile(input)
   if err != nil {
      log.Println(err)
      os.Exit(1)
   }

   // CSV data is read in columns - each line is a slice
   for _, line := range lines {
      temp := Record{
         Name:       line[0],
         Surname:    line[1],
         Number:     line[2],
         LastAccess: line[3],
      }
      myData = append(myData, temp)
      log.Println(temp)
   }

   err = saveCsvFile(output)
   if err != nil {
      log.Println(err)
      os.Exit(1)
   }
}
