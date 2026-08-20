package pdo

import (
   "fmt"

   "github.com/google/uuid"
)

type Company struct {
   _id      string
   _name    string
   _country string
}

func (aCompany *Company) SaveToDatabase() {
   fmt.Println("Saving company to database")
}

func (aCompany *Company) GetType() CompanyEnum {
   return CompanyType
}

func NewCompany(aName string, aCountry string) *Company {
   newId := uuid.New().String()
   return &Company{
      _id:      newId,
      _name:    aName,
      _country: aCountry}
}

func (aCompany *Company) GetId() string {
   return aCompany._id
}

func (aCompany *Company) SetId(anId string) {
   aCompany._id = anId
}

func (aCompany *Company) GetName() string {
   return aCompany._name
}

func (aCompany *Company) SetName(aName string) {
   aCompany._name = aName
}

func (aCompany *Company) GetCountry() string {
   return aCompany._country
}

func (aCompany *Company) SetCountry(aCountry string) {
   aCompany._country = aCountry
}
