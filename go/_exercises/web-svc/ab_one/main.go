/**
 * Example of run: ab_one -n 100 -c 16 -t 3 -u "https://metanit.com/"
 */

package main

import (
   "fmt"
   "sync"
   "time"

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
   concurrencyNum := viper.GetInt32(NumConcurrencyKey)
   timeoutInSec := viper.GetInt32(NumTimeoutKey)
   uriToTest := viper.GetString(UriToTestKey)

   var waitGrp sync.WaitGroup
   var guard = sync.Mutex{}
   allDurations := make([]time.Duration, 0)
   for i := int32(0); i < reqNum; i = i + concurrencyNum {
      for range concurrencyNum {
         waitGrp.Go(func() {
            accessDuration, err := benchUri(timeoutInSec, uriToTest)
            if err != nil {
               fmt.Println(err)
               return
            }

            guard.Lock()
            allDurations = append(allDurations, accessDuration)
            guard.Unlock()
         })
      }
   }

   waitGrp.Wait()
   meanDuration := meanTime(allDurations)
   fmt.Printf("Mean duration: '%v'\n", meanDuration)
}
