package main

import (
   "bytes"
   "net/http"
   "net/http/httptest"
   "strconv"
   "strings"
   "testing"
   "time"

   "github.com/gorilla/mux"
)

func TestTimeHandler(aTestCtx *testing.T) {
   request, err := http.NewRequest("GET", "/time", nil)
   if err != nil {
      aTestCtx.Fatal(err)
   }

   recorder := httptest.NewRecorder()
   handler := http.HandlerFunc(TimeHandler)
   handler.ServeHTTP(recorder, request)

   status := recorder.Code
   if status != http.StatusOK {
      aTestCtx.Errorf("handler returned wrong status code: got %v want %v",
         status,
         http.StatusOK)
   }
}

func TestMethodNotAllowed(aTestCtx *testing.T) {
   request, err := http.NewRequest("DELETE", "/time", nil)
   if err != nil {
      aTestCtx.Fatal(err)
   }

   recorder := httptest.NewRecorder()
   handler := http.HandlerFunc(MethodNotAllowedHandler)
   handler.ServeHTTP(recorder, request)

   status := recorder.Code
   if status != http.StatusNotFound {
      aTestCtx.Errorf("handler returned wrong status code: got %v want %v",
         status,
         http.StatusNotFound)
   }
}

func TestLogin(aTestCtx *testing.T) {
   userPass := []byte(`{"Username": "admin", "Password": "admin"}`)
   request, err := http.NewRequest("POST", "/login", bytes.NewBuffer(userPass))
   if err != nil {
      aTestCtx.Fatal(err)
   }

   request.Header.Set("Content-Type", "application/json")
   recorder := httptest.NewRecorder()
   handler := http.HandlerFunc(LoginHandler)
   handler.ServeHTTP(recorder, request)

   status := recorder.Code
   if status != http.StatusOK {
      aTestCtx.Errorf("handler returned wrong status code: got %v want %v",
         status,
         http.StatusOK)
      return
   }
}

func TestLogout(aTestCtx *testing.T) {
   userPass := []byte(`{"Username": "admin", "Password": "admin"}`)
   request, err := http.NewRequest("POST", "/logout", bytes.NewBuffer(userPass))
   if err != nil {
      aTestCtx.Fatal(err)
   }

   request.Header.Set("Content-Type", "application/json")
   recorder := httptest.NewRecorder()
   handler := http.HandlerFunc(LogoutHandler)
   handler.ServeHTTP(recorder, request)

   // Check the HTTP status code is what we expect.
   if status := recorder.Code; status != http.StatusOK {
      aTestCtx.Errorf("handler returned wrong status code: got %v want %v",
         status,
         http.StatusOK)
      return
   }
}

func TestAdd(aTestCtx *testing.T) {
   now := int(time.Now().Unix())
   username := "test_" + strconv.Itoa(now)
   users := `[{"Username": "admin", "Password": "admin"}, {"Username":"` + username + `", "Password": "myPass"}]`
   userPass := []byte(users)
   request, err := http.NewRequest("POST", "/add", bytes.NewBuffer(userPass))
   if err != nil {
      aTestCtx.Fatal(err)
   }

   request.Header.Set("Content-Type", "application/json")
   recorder := httptest.NewRecorder()
   handler := http.HandlerFunc(AddHandler)
   handler.ServeHTTP(recorder, request)

   // Check the HTTP status code is what we expect.
   if status := recorder.Code; status != http.StatusOK {
      aTestCtx.Errorf("handler returned wrong status code: got %v want %v",
         status,
         http.StatusOK)
      return
   }
}

func TestGetUserDataHandler(aTestCtx *testing.T) {
   userPass := []byte(`{"Username": "admin", "Password": "admin"}`)
   request, err := http.NewRequest("GET", "/username/1", bytes.NewBuffer(userPass))
   if err != nil {
      aTestCtx.Fatal(err)
   }

   request.Header.Set("Content-Type", "application/json")

   // `gorilla/mux` provides the `SetURLVars` function for testing purposes
   vars := map[string]string{
      "id": "1",
   }

   request = mux.SetURLVars(request, vars)
   recorder := httptest.NewRecorder()
   handler := http.HandlerFunc(GetUserDataHandler)
   handler.ServeHTTP(recorder, request)

   if status := recorder.Code; status != http.StatusOK {
      aTestCtx.Errorf("handler returned wrong status code: got %v want %v",
         status,
         http.StatusOK)
      return
   }

   expected := `{"id":1,"username":"admin","password":"admin","lastlogin":1702577035,"admin":1,"active":0}`
   serverResponse := recorder.Body.String()

   // result := strings.Split(serverResponse, "lastlogin")
   // serverResponse = result[0]
   serverResponse = strings.TrimSpace(serverResponse)

   if serverResponse != expected {
      aTestCtx.Errorf("handler returned unexpected body: got %v but wanted %v",
         serverResponse,
         expected)
   }
}
