package intRE

import (
   "regexp"
)

func matchInt(anInputString string) bool {
   bytes := []byte(anInputString)
   re := regexp.MustCompile(`^[-+]?\d+$`)
   return re.Match(bytes)
}
