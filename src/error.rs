#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Generic(#[from] color_eyre::Report),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),
    
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error)
}
