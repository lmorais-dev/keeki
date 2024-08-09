mod error;
mod prelude;
mod cli;
mod config;

use clap::Parser;

use crate::cli::KeekiCli;
use crate::cli::executor::CliExecutor;
use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let keeki_cli = KeekiCli::parse();
    CliExecutor::execute(keeki_cli).await?;
    
    Ok(())
}
