use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use task::Config;
use tracing::debug;

#[derive(Debug)]
pub enum CliError {
    InvalidUsage,
    InvalidNumber(ParseIntError),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidNumber(_) => write!(f, "cannot parse given number"),
            CliError::InvalidUsage => write!(f, "expected number as the first argument"),
        }
    }
}

impl From<ParseIntError> for CliError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidNumber(value)
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CliError::InvalidNumber(e) => Some(e),
            _ => None,
        }
    }
}

pub fn env_config() -> Result<Config, CliError> {
    let input = env::args().nth(1).ok_or(CliError::InvalidUsage)?;
    let num: u64 = input.parse()?;

    debug!("Creating config with number {num}");

    Ok(Config::new(num))
}

pub type RunFn = fn() -> Result<(), CliError>;

pub fn exec(fun: RunFn) {
    let Err(e) = fun() else {
        return;
    };

    eprintln!("error occurred: {e}");

    if let Some(source) = e.source() {
        eprintln!("\tcaused by: {source}");
    }
}
