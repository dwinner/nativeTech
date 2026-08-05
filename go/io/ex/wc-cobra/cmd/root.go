package cmd

import (
   "github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
   Use:   "wc_cobra",
   Short: "UNIX-like word count cli-tool",
   Long:  "UNIX-like word count cli-tool",
}

func Execute() {
   cobra.CheckErr(rootCmd.Execute())
}

func init() {
}
