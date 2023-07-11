use clap::Parser;

mod commands;
mod types;
mod utils;

use types::{commands::Args, commands::Commands, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Commands::AddCertificate {
                cert_path,
                key_path,
            } => commands::config::call(cert_path, key_path)?,
            Commands::Serve => commands::serve::call(args.port).await?,
        }
    } else {
        // Default action is to start proxyl server
        commands::serve::call(args.port).await?;
    }

    Ok(())
}
