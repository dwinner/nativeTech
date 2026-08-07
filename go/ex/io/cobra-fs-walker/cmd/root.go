package cmd

import (
   "github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
   Use:   "cobra_fs_walker",
   Short: "Simple file system utility",
   Long:  "Simple file system utility",
}

func Execute() {
   cobra.CheckErr(rootCmd.Execute())
}

func init() {
}
