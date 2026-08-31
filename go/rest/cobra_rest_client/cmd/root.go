package cmd

import (
   "bytes"
   "encoding/json"
   "fmt"
   "io"
   "os"

   "github.com/spf13/cobra"
)

var Server string
var Port string
var data string
var username string
var password string

type User struct {
   ID        int    `json:"id"`
   Username  string `json:"username"`
   Password  string `json:"password"`
   LastLogin int64  `json:"lastlogin"`
   Admin     int    `json:"admin"`
   Active    int    `json:"active"`
}

const (
   empty = ""
   tab   = "\t"
)

func (aUser *User) FromJson(reader io.Reader) error {
   decoder := json.NewDecoder(reader)
   return decoder.Decode(aUser)
}

func (aUser *User) ToJson(writer io.Writer) error {
   encoder := json.NewEncoder(writer)
   return encoder.Encode(aUser)
}

// SliceFromJson decodes a serialized slice with JSON records
func SliceFromJson(slice any, reader io.Reader) error {
   decoder := json.NewDecoder(reader)
   return decoder.Decode(slice)
}

// SliceToJson encodes a slice with JSON records
func SliceToJson(slice any, writer io.Writer) error {
   encoder := json.NewEncoder(writer)
   return encoder.Encode(slice)
}

// PrettyJson is for pretty printing JSON records
func PrettyJson(data any) (string, error) {
   buffer := new(bytes.Buffer)
   encoder := json.NewEncoder(buffer)
   encoder.SetIndent(empty, tab)
   err := encoder.Encode(data)
   if err != nil {
      return empty, err
   }

   return buffer.String(), nil
}

var rootCmd = &cobra.Command{
   Use:   "rest-cli",
   Short: "A REST API client",
   Long:  `A Client for a RESTful server.`,
}

func Execute() {
   if err := rootCmd.Execute(); err != nil {
      fmt.Println(err)
      os.Exit(1)
   }
}

func init() {
   rootCmd.PersistentFlags().StringVarP(&username, "username", "u", "username", "The username")
   rootCmd.PersistentFlags().StringVarP(&password, "password", "p", "admin", "The password")
   rootCmd.PersistentFlags().StringVarP(&data, "data", "d", "{}", "JSON Record")
   rootCmd.PersistentFlags().StringVarP(&Server, "server", "s", "http://localhost", "RESTful server hostname")
   rootCmd.PersistentFlags().StringVarP(&Port, "port", "P", ":1234", "Port of RESTful Server")
}
