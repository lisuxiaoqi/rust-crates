use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug)]
struct LowError {
    source: std::io::Error,
}
impl std::error::Error for LowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
impl std::fmt::Display for LowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LowError wrapper for the std error",)
    }
}

fn read_config(path: PathBuf) -> Result<String, LowError> {
    std::fs::read_to_string(&path).map_err(|e| LowError { source: e })
}

#[derive(Debug)]
#[allow(unused)]
enum HighError {
    ConfigError(LowError),
    AnyWay,
}

impl std::error::Error for HighError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HighError::ConfigError(e) => Some(e),
            HighError::AnyWay => None,
        }
    }
}
impl Display for HighError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HighError::ConfigError(_) => write!(f, "HighError wrapper for ConfigError"),
            HighError::AnyWay => write!(f, "HighError AnyWay"),
        }
    }
}

impl From<LowError> for HighError {
    fn from(value: LowError) -> Self {
        HighError::ConfigError(value)
    }
}

fn init_app() -> Result<(), HighError> {
    let _ = read_config("none.config".into())?;
    Ok(())
}

#[test]
fn error_() {
    if let Err(e) = init_app() {
        let mut current = e.source();
        println!("error:{}", e);
        while let Some(sub) = current {
            println!("\tcaused by:{}", sub);
            current = sub.source();
        }
    }
}
