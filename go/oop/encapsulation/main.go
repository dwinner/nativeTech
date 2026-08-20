package main

import "encapsulation/pdo"

func main() {
   company := pdo.NewCompany("MyCompany", "India")
   company.SaveToDatabase()
}
