package main

import (
   "log"
   "net/http"
   "os"
   "os/signal"
   "time"

   "github.com/gorilla/mux"
)

// Router Create a new ServeMux using Gorilla
var Router = mux.NewRouter()

// Port is where the web server listens to
var Port = ":1234"

func main() {
   arguments := os.Args
   if len(arguments) >= 2 {
      Port = ":" + arguments[1]
   }

   httpServer := http.Server{
      Addr:         Port,
      Handler:      Router,
      ErrorLog:     nil,
      ReadTimeout:  5 * time.Second,
      WriteTimeout: 5 * time.Second,
      IdleTimeout:  10 * time.Second,
   }

   Router.NotFoundHandler = http.HandlerFunc(DefaultHandler)
   notAllowed := notAllowedHandler{}
   Router.MethodNotAllowedHandler = notAllowed
   Router.HandleFunc("/time", TimeHandler)

   // Define Handler Functions
   // Register GET
   getMux := Router.Methods(http.MethodGet).Subrouter()

   getMux.HandleFunc("/getall", GetAllHandler)
   getMux.HandleFunc("/getid/{username}", GetIdHandler)
   getMux.HandleFunc("/logged", LoggedUsersHandler)
   getMux.HandleFunc("/username/{id:[0-9]+}", GetUserDataHandler)

   // Register PUT
   // Update User
   putMux := Router.Methods(http.MethodPut).Subrouter()
   putMux.HandleFunc("/update", UpdateHandler)

   // Register POST
   // Add User + Login + Logout
   postMux := Router.Methods(http.MethodPost).Subrouter()
   postMux.HandleFunc("/add", AddHandler)
   postMux.HandleFunc("/login", LoginHandler)
   postMux.HandleFunc("/logout", LogoutHandler)

   // Register DELETE User
   deleteMux := Router.Methods(http.MethodDelete).Subrouter()
   deleteMux.HandleFunc("/username/{id:[0-9]+}", DeleteHandler)

   go func() {
      log.Println("Listening to", Port)
      err := httpServer.ListenAndServe()
      if err != nil {
         log.Printf("Error starting server: %s\n", err)
         return
      }
   }()

   sigs := make(chan os.Signal, 1)
   signal.Notify(sigs, os.Interrupt)
   sig := <-sigs
   log.Println("Quitting after signal:", sig)
   time.Sleep(5 * time.Second)
   _ = httpServer.Shutdown(nil)
}
