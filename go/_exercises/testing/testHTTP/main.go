package main

import (
   "log"
   "net/http"
   "os"
   "os/signal"
   "time"

   "github.com/gorilla/mux"
)

// Create a new ServeMux using Gorilla
var router = mux.NewRouter()

// Port is where the web server listens to
var Port = ":1234"

func main() {
   arguments := os.Args
   if len(arguments) >= 2 {
      Port = ":" + arguments[1]
   }

   httpServer := http.Server{
      Addr:         Port,
      Handler:      router,
      ErrorLog:     nil,
      ReadTimeout:  5 * time.Second,
      WriteTimeout: 5 * time.Second,
      IdleTimeout:  10 * time.Second,
   }

   router.NotFoundHandler = http.HandlerFunc(DefaultHandler)
   notAllowed := notAllowedHandler{}
   router.MethodNotAllowedHandler = notAllowed

   router.HandleFunc("/time", TimeHandler)

   // Define Handler Functions
   // Register GET
   getSubRouter := router.Methods(http.MethodGet).Subrouter()
   getSubRouter.HandleFunc("/getall", GetAllHandler)
   getSubRouter.HandleFunc("/getid/{username}", GetIdHandler)
   getSubRouter.HandleFunc("/logged", LoggedUsersHandler)
   getSubRouter.HandleFunc("/username/{id:[0-9]+}", GetUserDataHandler)

   // Register PUT
   // Update User
   putSubRouter := router.Methods(http.MethodPut).Subrouter()
   putSubRouter.HandleFunc("/update", UpdateHandler)

   // Register POST
   // Add User + Login + Logout
   postSubRouter := router.Methods(http.MethodPost).Subrouter()
   postSubRouter.HandleFunc("/add", AddHandler)
   postSubRouter.HandleFunc("/login", LoginHandler)
   postSubRouter.HandleFunc("/logout", LogoutHandler)

   // Register DELETE
   deleteSubRouter := router.Methods(http.MethodDelete).Subrouter()
   deleteSubRouter.HandleFunc("/username/{id:[0-9]+}", DeleteHandler)

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
