package pdo

import "fmt"

type Vendor struct {
   Company
   AccountNumber string
}

func NewVendor(aCompanyName string, aCompanyCountry string, anAccountNumber string) *Vendor {
   company := NewCompany(aCompanyName, aCompanyCountry)
   vendor := &Vendor{
      Company:       *company,
      AccountNumber: anAccountNumber,
   }

   vendor.SaveToDatabase()
   return vendor
}

func (aVendor *Vendor) SaveToDatabase() {
   fmt.Println("Saving vendor to database")
}

func (aVendor *Vendor) GetType() CompanyEnum {
   return VendorType
}
