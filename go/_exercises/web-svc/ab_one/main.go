/**
 * Example of run: ab_one -n 100 -c 16 -t 3 -u "https://metanit.com/"
 */

package main

import (
   "fmt"

   "github.com/spf13/pflag"
   "github.com/spf13/viper"
)

const (
   DefaultRequestNumber     = 8
   DefaultConcurrencyNumber = 4
   DefaultTimeoutInSec      = 5
   NumReqKey                = "requests"
   NumConcurrencyKey        = "concurrency"
   NumTimeoutKey            = "timeout"
   UriToTestKey             = "URI"
)

func main() {
   // Set up command line arguments
   pflag.Int32P(NumReqKey, "n", DefaultRequestNumber, "Number of requests to perform")
   pflag.Int32P(NumConcurrencyKey, "c", DefaultConcurrencyNumber, "Number of multiple requests to make")
   pflag.Int32P(NumTimeoutKey, "t", DefaultTimeoutInSec, "Seconds to max. wait for responses")
   pflag.StringP(UriToTestKey, "u", "", "URI to web resource")
   pflag.Parse()
   _ = viper.BindPFlags(pflag.CommandLine)

   reqNum := viper.GetInt32(NumReqKey)
   //concurrencyNum := viper.GetInt32(NumConcurrencyKey)
   timeoutInSec := viper.GetInt32(NumTimeoutKey)
   uriToTest := viper.GetString(UriToTestKey)

   for range reqNum {
      accessDuration, err := benchUri(timeoutInSec, uriToTest)
      if err != nil {
         fmt.Println(err)
      } else {
         fmt.Printf("Page took '%v'\n", accessDuration)
      }
   }
}
