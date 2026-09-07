package main

import (
   "testing"
   "unicode/utf8"
)

func TestR1(aTestCtx *testing.T) {
   testCases := []struct {
      in, want string
   }{
      {" ", " "},
      {"!12345@", "@54321!"},
      {"Mastering Go", "oG gniretsaM"},
   }

   for _, testCase := range testCases {
      rev := Reverse1(testCase.in)
      if string(rev) != testCase.want {
         aTestCtx.Errorf("Reverse: %q, want %q", rev, testCase.want)
      }
   }
}

func TestR2(aTestCtx *testing.T) {
   testCases := []struct {
      in, want string
   }{
      {" ", " "},
      {"!12345@", "@54321!"},
      {"Mastering Go", "oG gniretsaM"},
   }

   for _, testCase := range testCases {
      rev := Reverse2(testCase.in)
      if rev != testCase.want {
         aTestCtx.Errorf("Reverse: %q, want %q", rev, testCase.want)
      }
   }
}

func FuzzR1(aFuzzCtx *testing.F) {
   testCases := []string{"Hello, world", " ", "!12345"}
   for _, testCase := range testCases {
      aFuzzCtx.Add(testCase)
   }

   aFuzzCtx.Fuzz(func(testing *testing.T, orig string) {
      rev := Reverse1(orig)
      doubleRev := Reverse1(string(rev))
      if orig != string(doubleRev) {
         testing.Errorf("Before: %q, after: %q", orig, doubleRev)
      }

      if utf8.ValidString(orig) && !utf8.ValidString(string(rev)) {
         testing.Errorf("Reverse: invalid UTF-8 string %q", rev)
      }
   })
}

func FuzzR2(aFuzzCtx *testing.F) {
   testCases := []string{"Hello, world", " ", "!12345"}
   for _, testCase := range testCases {
      aFuzzCtx.Add(testCase)
   }

   aFuzzCtx.Fuzz(func(testing *testing.T, orig string) {
      rev := Reverse2(orig)
      doubleRev := Reverse2(rev)
      if orig != doubleRev {
         testing.Errorf("Before: %q, after: %q", orig, doubleRev)
      }

      if utf8.ValidString(orig) && !utf8.ValidString(rev) {
         testing.Errorf("Reverse: invalid UTF-8 string %q", rev)
      }
   })
}
