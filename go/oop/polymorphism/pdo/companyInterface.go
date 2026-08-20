package pdo

type CompanyInterface interface {
   SaveToDatabase()
   GetType() CompanyEnum
}
