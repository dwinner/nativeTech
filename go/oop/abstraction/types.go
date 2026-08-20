package main

import (
   "errors"

   "github.com/google/uuid"
)

type CompanyEntity struct {
   Id      string
   Name    string
   Country string
   manager string
}

func (aCompany *CompanyEntity) saveToDatabase() error {
   return errors.New("not implemented")
}

func (aCompany *CompanyEntity) getType() error {
   return errors.New("not implemented")
}

// CompanyInterface Define an interface for types that can be saved to the database
type CompanyInterface interface {
   saveToDatabase()
   getType() CompanyEnum
}

type CompanyEnum string

const (
   CompanyType  CompanyEnum = "Company"
   VendorType   CompanyEnum = "Vendor"
   EmployeeType CompanyEnum = "Employee"
)

type Company struct {
   CompanyEntity
}

type Employee struct {
   CompanyEntity
   Salary int
}

type VendorCompany struct {
   CompanyEntity
   AccountNumber string
}

func newCompanyEntity(aName string, aCountry string) CompanyEntity {
   return CompanyEntity{
      Id:      uuid.New().String(),
      Name:    aName,
      Country: aCountry,
   }
}

func newCompany(aName string, aCountry string) Company {
   companyEntity := newCompanyEntity(aName, aCountry)
   return Company{
      companyEntity,
   }
}

func newEmployee(aCompanyName string, aCompanyCountry string, aSalary int) Employee {
   companyEntity := newCompanyEntity(aCompanyName, aCompanyCountry)
   employee := Employee{
      CompanyEntity: companyEntity,
      Salary:        aSalary,
   }

   _ = employee.saveToDatabase()
   return employee
}

func newVendor(aCompanyName string, aCompanyCountry string, anAccountNumber string) VendorCompany {
   companyEntity := newCompanyEntity(aCompanyName, aCompanyCountry)
   vendor := VendorCompany{
      CompanyEntity: companyEntity,
      AccountNumber: anAccountNumber,
   }

   _ = vendor.saveToDatabase()
   return vendor
}
