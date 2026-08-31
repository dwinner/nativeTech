package main

import (
   "bytes"
   "encoding/json"
   "fmt"
   "io"
   "net/http"
   "time"
)

type User struct {
   Username string `json:"user"`
   Password string `json:"password"`
}

var User1 = User{"admin", "admin"}
var User2 = User{"tsoukalos", "pass"}
var User3 = User{"", "pass"}

func deleteEndpoint(aServer string, aUser User) int {
   encodedUser, err := json.Marshal(aUser)
   if err != nil {
      fmt.Println("Error in req: ", err)
      return http.StatusBadRequest
   }

   reader := bytes.NewReader(encodedUser)
   request, err := http.NewRequest(http.MethodDelete, aServer+deleteEndPoint, reader)
   if err != nil {
      fmt.Println("Error in req: ", err)
      return http.StatusBadRequest
   }

   request.Header.Set("Content-Type", "application/json")
   httpClient := &http.Client{
      Timeout: 15 * time.Second,
   }

   response, err := httpClient.Do(request)
   if err != nil {
      fmt.Println("Error:", err)
   }

   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(response.Body)

   data, err := io.ReadAll(response.Body)
   fmt.Print("/delete returned: ", string(data))
   if err != nil {
      fmt.Println("Error:", err)
   }

   return response.StatusCode
}

func getEndpoint(aServer string, aUser User) int {
   encodedUser, err := json.Marshal(aUser)
   if err != nil {
      fmt.Println("Error in unmarshalling: ", err)
      return http.StatusBadRequest
   }

   reader := bytes.NewReader(encodedUser)
   req, err := http.NewRequest(http.MethodGet, aServer+getEndPoint, reader)
   if err != nil {
      fmt.Println("Error in req: ", err)
      return http.StatusBadRequest
   }

   req.Header.Set("Content-Type", "application/json")
   httpClient := &http.Client{
      Timeout: 15 * time.Second,
   }

   response, err := httpClient.Do(req)
   if err != nil {
      fmt.Println("Error:", err)
   }

   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(response.Body)

   data, err := io.ReadAll(response.Body)
   fmt.Print("/get returned: ", string(data))
   if err != nil {
      fmt.Println("Error:", err)
   }

   return response.StatusCode
}

func addEndpoint(aServer string, aUser User) int {
   encodedUser, err := json.Marshal(aUser)
   if err != nil {
      fmt.Println("Error in unmarshalling: ", err)
      return http.StatusBadRequest
   }

   reader := bytes.NewReader(encodedUser)
   request, err := http.NewRequest(http.MethodPost, aServer+addEndPoint, reader)
   if err != nil {
      fmt.Println("Error in req: ", err)
      return http.StatusBadRequest
   }

   request.Header.Set("Content-Type", "application/json")
   httpClient := &http.Client{
      Timeout: 15 * time.Second,
   }

   response, err := httpClient.Do(request)
   if err != nil {
      return http.StatusNotFound
   }

   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(response.Body)

   return response.StatusCode
}

func timeEndpoint(aServer string) (int, string) {
   request, err := http.NewRequest(http.MethodPost, aServer+timeEndPoint, nil)
   if err != nil {
      fmt.Println("Error in req: ", err)
      return http.StatusBadRequest, ""
   }

   httpClient := &http.Client{
      Timeout: 15 * time.Second,
   }

   response, err := httpClient.Do(request)
   if err != nil {
      return http.StatusBadRequest, ""
   }

   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(response.Body)

   data, _ := io.ReadAll(response.Body)
   return response.StatusCode, string(data)
}

func slashEndpoint(aServer, aUrl string) (int, string) {
   request, err := http.NewRequest(http.MethodPost, aServer+aUrl, nil)
   if err != nil {
      fmt.Println("Error in req: ", err)
      return http.StatusBadRequest, ""
   }

   httpClient := &http.Client{
      Timeout: 15 * time.Second,
   }

   response, err := httpClient.Do(request)
   if err != nil {
      return http.StatusBadRequest, ""
   }

   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(response.Body)

   data, _ := io.ReadAll(response.Body)
   return response.StatusCode, string(data)
}

const addEndPoint = "/add"
const getEndPoint = "/get"
const deleteEndPoint = "/delete"
const timeEndPoint = "/time"

func main() {
   server := "localhost:1234"

   fmt.Println("/add")
   httpCode := addEndpoint(server, User1)
   if httpCode != http.StatusOK {
      fmt.Println("u1 Return code:", httpCode)
   } else {
      fmt.Println("u1 Data added:", User1, httpCode)
   }

   httpCode = addEndpoint(server, User2)
   if httpCode != http.StatusOK {
      fmt.Println("u2 Return code:", httpCode)
   } else {
      fmt.Println("u2 Data added:", User2, httpCode)
   }

   httpCode = addEndpoint(server, User3)
   if httpCode != http.StatusOK {
      fmt.Println("u3 Return code:", httpCode)
   } else {
      fmt.Println("u3 Data added:", User3, httpCode)
   }

   fmt.Println("/get")
   httpCode = getEndpoint(server, User1)
   fmt.Println("/get u1 return code:", httpCode)
   httpCode = getEndpoint(server, User2)
   fmt.Println("/get u2 return code:", httpCode)
   httpCode = getEndpoint(server, User3)
   fmt.Println("/get u3 return code:", httpCode)

   fmt.Println("/delete")
   httpCode = deleteEndpoint(server, User1)
   fmt.Println("/delete u1 return code:", httpCode)
   httpCode = deleteEndpoint(server, User1)
   fmt.Println("/delete u1 return code:", httpCode)
   httpCode = deleteEndpoint(server, User2)
   fmt.Println("/delete u2 return code:", httpCode)
   httpCode = deleteEndpoint(server, User3)
   fmt.Println("/delete u3 return code:", httpCode)

   fmt.Println("/time")
   httpCode, myTime := timeEndpoint(server)
   fmt.Print("/time returned: ", httpCode, " ", myTime)
   time.Sleep(time.Second)
   httpCode, myTime = timeEndpoint(server)
   fmt.Print("/time returned: ", httpCode, " ", myTime)

   fmt.Println("/")
   URL := "/"
   httpCode, response := slashEndpoint(server, URL)
   fmt.Print("/ returned: ", httpCode, " with response: ", response)

   fmt.Println("/what")
   URL = "/what"
   httpCode, response = slashEndpoint(server, URL)
   fmt.Print(URL, " returned: ", httpCode, " with response: ", response)
}
