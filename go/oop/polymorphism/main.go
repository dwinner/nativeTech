package main

import (
   "fmt"
   "polymorphism/pdo"
)

func main() {
   newCompany := pdo.NewCompany("Lakshmi Chit Funds", "India")
   employee := pdo.NewEmployee(newCompany, 460_000)
   vendor := pdo.NewVendor("Babu Rao", "India", "abcd1234")

   var companies = make([]pdo.CompanyInterface, 0)
   companies = append(companies, newCompany)
   companies = append(companies, employee)
   companies = append(companies, vendor)

   for _, element := range companies {
      printCompany(element)
   }
}

func printCompany(aCompany pdo.CompanyInterface) {
   if aCompany != nil {
      fmt.Println(aCompany.GetType())
   }
}
