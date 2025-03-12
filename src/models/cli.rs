use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "A command line tool for managing Cline rules and modes")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Generate,
}
