package main

import (
   "context"
   "io"
   "net/http"
   "time"
)

func benchUri(aTimeoutSec int32, aUriToTest string) (duration time.Duration, err error) {
   ctx, cancelFunc := context.WithTimeout(context.Background(), time.Second*time.Duration(aTimeoutSec))
   defer cancelFunc()

   request, requestErr := http.NewRequestWithContext(ctx, http.MethodGet, aUriToTest, nil)
   if requestErr != nil {
      return 0, requestErr
   }

   start := time.Now()
   response, responseErr := http.DefaultClient.Do(request.WithContext(ctx))
   if responseErr != nil {
      return 0, responseErr
   }

   duration = time.Since(start)
   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(response.Body)

   return duration, nil
}

func meanTime(aDurationSlice []time.Duration) time.Duration {
   sum := time.Duration(0)
   for _, duration := range aDurationSlice {
      sum = sum + duration
   }

   avg := sum / time.Duration(len(aDurationSlice))
   return avg
}
