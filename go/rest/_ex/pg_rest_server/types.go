package main

import (
   "encoding/json"
   "fmt"
   "io"
   "log"
   "net/http"
)

type User struct {
   Id        int    `json:"id"`
   Username  string `json:"username"`
   Password  string `json:"password"`
   LastLogin int64  `json:"lastlogin"`
   Admin     int    `json:"admin"`
   Active    int    `json:"active"`
}

// SliceToJson encodes a slice with JSON records
func SliceToJson(aSlice any, aWriter io.Writer) error {
   encoder := json.NewEncoder(aWriter)
   return encoder.Encode(aSlice)
}

type notAllowedHandler struct{}

func (aHandler notAllowedHandler) ServeHTTP(responseWriter http.ResponseWriter, request *http.Request) {
   MethodNotAllowedHandler(responseWriter, request)
}

func DefaultHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("DefaultHandler Serving:", request.URL.Path, "from", request.Host, "with method", request.Method)
   responseWriter.WriteHeader(http.StatusNotFound)
   body := request.URL.Path + " is not supported. Thanks for visiting!\n"
   _, _ = fmt.Fprintf(responseWriter, "%s", body)
}

// MethodNotAllowedHandler is executed when the HTTP method is incorrect
func MethodNotAllowedHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("Serving:", request.URL.Path, "from", request.Host, "with method", request.Method)
   responseWriter.WriteHeader(http.StatusNotFound)
   body := "Method not allowed!\n"
   _, _ = fmt.Fprintf(responseWriter, "%s", body)
}
