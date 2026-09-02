package main

import (
   "fmt"
   "net/http"
   "strings"

   "github.com/gorilla/mux"
)

func handler(_ http.ResponseWriter, _ *http.Request) {
   return
}

func (h notAllowedHandler) ServeHTTP(writer http.ResponseWriter, request *http.Request) {
   handler(writer, request)
}

type notAllowedHandler struct{}

func main() {
   router := mux.NewRouter()
   router.NotFoundHandler = http.HandlerFunc(handler)
   notAllowed := notAllowedHandler{}
   router.MethodNotAllowedHandler = notAllowed

   // Register GET
   getSubRouter := router.Methods(http.MethodGet).Subrouter()
   getSubRouter.HandleFunc("/time", handler)
   getSubRouter.HandleFunc("/getall", handler)
   getSubRouter.HandleFunc("/getid", handler)
   getSubRouter.HandleFunc("/logged", handler)
   getSubRouter.HandleFunc("/username/{id:[0-9]+}", handler)

   // Register PUT
   // Update User
   putSubRouter := router.Methods(http.MethodPut).Subrouter()
   putSubRouter.HandleFunc("/update", handler)

   // Register POST
   // Add User + Login + Logout
   postSubRouter := router.Methods(http.MethodPost).Subrouter()
   postSubRouter.HandleFunc("/add", handler)
   postSubRouter.HandleFunc("/login", handler)
   postSubRouter.HandleFunc("/logout", handler)

   // Register DELETE
   // Delete User
   deleteSubRouter := router.Methods(http.MethodDelete).Subrouter()
   deleteSubRouter.HandleFunc("/username/{id:[0-9]+}", handler)

   err := router.Walk(func(route *mux.Route, router *mux.Router, ancestors []*mux.Route) error {
      pathTemplate, err := route.GetPathTemplate()
      if err == nil {
         fmt.Println("ROUTE:", pathTemplate)
      }

      pathRegexp, err := route.GetPathRegexp()
      if err == nil {
         fmt.Println("Path regexp:", pathRegexp)
      }

      qT, err := route.GetQueriesTemplates()
      if err == nil {
         fmt.Println("Queries templates:", strings.Join(qT, ","))
      }

      qRegexps, err := route.GetQueriesRegexp()
      if err == nil {
         fmt.Println("Queries regexps:", strings.Join(qRegexps, ","))
      }

      methods, err := route.GetMethods()
      if err == nil {
         fmt.Println("Methods:", strings.Join(methods, ","))
      }
		
      fmt.Println()
      return nil
   })

   if err != nil {
      fmt.Println(err)
   }

   http.Handle("/", router)
}
