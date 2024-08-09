use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;
use crate::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub exclusions: Vec<String>,
    pub ollama: OllamaConfig
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub url: String,
    pub model: String
}

impl ProjectConfig {
    pub async fn load(config_path: String) -> Result<Self> {
        let mut file = tokio::fs::File::open(config_path.as_str()).await?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).await?;
        
        let project_config = toml::from_str(file_content.as_str())?;
        
        Ok(project_config)
    }
    
    pub async fn save(&self, config_path: String, overwrite: bool) -> Result<()> {
        let mut open_options = OpenOptions::new();
        open_options.write(true);
        open_options.create_new(true);
        
        if overwrite {
            open_options.create_new(false);
            open_options.create(true);
            open_options.truncate(true);
        }
        
        let mut config_file = open_options.open(config_path).await?;
        let config_file_content = toml::to_string_pretty(self)?;
        config_file.write_all(config_file_content.as_bytes()).await?;
        
        Ok(())
    }
}