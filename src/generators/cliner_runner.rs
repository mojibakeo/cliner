use crate::error::Result;
use crate::models::{Cli, Commands};
use crate::generators::{ClinerGenerator, ClinerInitializer};
use clap::{CommandFactory, Parser};

pub struct ClinerRunner;

impl ClinerRunner {
    pub fn show_help() {
        let mut cli = Cli::command();
        cli.print_help().expect("ヘルプの表示に失敗したのだ");
    }
    
    pub fn run() -> Result<()> {
        let cli = Cli::parse();
        
        match cli.command {
            Some(Commands::Init) => {
                let initializer = ClinerInitializer::new();
                initializer.run_init()
            },
            Some(Commands::Generate) => {
                let generator = ClinerGenerator::new();
                generator.run_generate()
            },
            None => {
                Self::show_help();
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_show_help() {
        ClinerRunner::show_help();
    }
    
    #[test]
    fn test_run_with_no_args() {
        let result = ClinerRunner::run();
        assert!(result.is_ok());
    }
}
