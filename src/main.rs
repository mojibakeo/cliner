mod models;
mod error;
mod processors;
mod generators;

use error::Result;
use generators::ClinerRunner;

fn main() -> Result<()> {
   ClinerRunner::run()
}
