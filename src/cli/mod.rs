pub mod executor;

use clap::{Parser, Subcommand};

#[derive(Clone, Parser)]
#[command(version, about, arg_required_else_help = true)]
pub struct KeekiCli {
    #[command(subcommand)]
    command: KeekiCommand
}

#[derive(Clone, Subcommand)]
pub enum KeekiCommand {
    /// Initializes a new project in the current folder
    Init,

    /// Setup the index database
    Setup,

    /// Index all project sources and related files
    Index,

    /// Generate embeddings for indexed sources and files
    Process,

    /// Chat with the LLM
    Chat
}