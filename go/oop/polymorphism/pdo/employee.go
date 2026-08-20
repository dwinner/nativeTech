package pdo

import "fmt"

type Employee struct {
   Company // make it anonymous
   salary  int
}

func NewEmployee(aCompany *Company, aSalary int) *Employee {
   newEmp := &Employee{
      Company: *aCompany,
      salary:  aSalary}
   newEmp.SaveToDatabase() // Now we have access the Company type's methods
   return newEmp
}

func (anEmployee *Employee) GetSalary() int {
   return anEmployee.salary
}

func (anEmployee *Employee) SetSalary(salary int) {
   anEmployee.salary = salary
}

func (anEmployee *Employee) SaveToDatabase() {
   fmt.Println("Saving employee to database")
}

func (anEmployee *Employee) GetType() CompanyEnum {
   return EmployeeType
}
