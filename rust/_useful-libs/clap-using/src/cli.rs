use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
   author = "dwinner@inbox.ru",
   version = "1.0",
   about = "A command-line text manipulation utility",
   name = "Text Manipulation Utility"
)]
pub struct Cli
{
   /// Turn debugging information on.
   #[arg(short, long, action = clap::ArgAction::Count)]
   debug: u8,

   /// Find and Replace commands.
   #[command(subcommand)]
   pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands
{
   /// Subcommand for handling find operations.
   Find(FindCommands),

   /// Subcommand for handling replace operations.
   Replace(ReplaceCommands),
}

/// Represents find-related commands.
#[derive(Debug, Args)]
pub struct FindCommands
{
   /// Sets the input file to process
   #[clap(short = 'i', long = "input", num_args = 1.., value_delimiter = ' ')]
   pub input: Option<Vec<String>>,

   /// Sets the pattern to find.
   #[clap(short = 'p', long = "pattern")]
   pub pattern: Option<String>,
}

/// Represents replace-related commands.
#[derive(Debug, Args)]
pub struct ReplaceCommands
{
   /// Sets the input file to process
   #[clap(short = 'i', long = "input", num_args = 1.., value_delimiter = ' ')]
   pub input: Option<Vec<String>>,

   /// Sets the pattern to find.
   #[clap(short = 'p', long = "pattern")]
   pub pattern: Option<String>,

   /// Sets the replacement text.
   #[clap(short = 'r', long = "replace")]
   pub replace: Option<String>,

   /// Perform a case-insensitive search and replace.
   #[clap(short = 'c', long = "ignore-case")]
   pub ignore_case: bool,
}
