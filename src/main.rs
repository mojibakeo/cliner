mod models;
mod error;
mod processors;
mod generators;

use error::Result;
use generators::ClinerGenerator;

fn main() -> Result<()> {
   ClinerGenerator::new().run()
}
