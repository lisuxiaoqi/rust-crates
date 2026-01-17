use anyhow::{Context, Result};
use std::error::Error;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("LowError wrapper for source")]
struct LowError {
    #[source]
    source: std::io::Error,
}

fn read_config(path: PathBuf) -> Result<String, LowError> {
    std::fs::read_to_string(&path).map_err(|e| LowError { source: e })
}

#[derive(Debug, Error)]
#[allow(unused)]
enum HighError {
    #[error("ConfigError")]
    ConfigError(#[from] LowError),
    #[error("Error Anyway")]
    AnyWay,
}
fn init_app() -> Result<(), HighError> {
    let _ = read_config("none.config".into())?;
    Ok(())
}

#[test]
fn error_test() {
    if let Err(e) = init_app() {
        let mut current = e.source();
        println!("error:{}", e);
        while let Some(sub) = current {
            println!("\tcaused by:{}", sub);
            current = sub.source();
        }
    }
}

fn init_config() -> Result<(), HighError> {
    let _ = read_config("none.config".into())?;
    Ok(())
}
fn init_app_anyhow() -> anyhow::Result<()> {
    init_config().context("AnyHow context for HighError")
}

#[test]
fn error_test_anyhow() {
    if let Err(e) = init_app_anyhow() {
        if let Some(c) = e.downcast_ref::<HighError>() {
            println!("Downcast anyhow to concrate error:{}", c);
        }

        let mut current = e.source();
        println!("anyhow error:{}", e);
        while let Some(sub) = current {
            println!("\tcaused by:{}", sub);
            current = sub.source();
        }
    }
}
