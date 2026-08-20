package main

import "inheritance/pdo"

func main() {
   newCompany := pdo.NewCompany("Bosh", "GE")
   employee := pdo.NewEmployee(newCompany, 470_000)
   employee.SaveToDatabase()
}
