package main

import (
   "github.com/gin-gonic/gin"
)

var Address = "localhost:1234"

func main() {
   router := gin.Default()
   router.GET("/", handleDefault)
   router.GET("/time", handleTime)
   router.POST("/add", handleAdd)
   router.GET("/get", handleGet)
   router.DELETE("/delete", handleDelete)
   _ = router.Run(Address)
}
