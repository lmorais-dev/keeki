pub struct CliExecutor;

use crate::cli::KeekiCommand;
use crate::config::{OllamaConfig, ProjectConfig};
use crate::prelude::*;

impl CliExecutor {
    pub async fn execute(cli: super::KeekiCli) -> Result<()> {
        let current_directory = std::env::current_dir()?;

        match &cli.command {
            KeekiCommand::Init => {
                let config_path = current_directory.join("keeki.toml").to_string_lossy().to_string();

                let project_config = ProjectConfig {
                    exclusions: vec![],
                    ollama: OllamaConfig {
                        url: "http://localhost:11434".to_string(),
                        model: "dolphincoder:7b-starcoder2-q6_K".to_string(),
                    }
                };
                
                project_config.save(config_path, true).await?;
            }
            KeekiCommand::Setup => {}
            KeekiCommand::Index => {}
            KeekiCommand::Process => {}
            KeekiCommand::Chat => {}
        }

        Ok(())
    }
}