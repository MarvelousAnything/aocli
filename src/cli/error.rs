use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("CLI error")]
#[diagnostic(code(cli::error))]
pub enum CliError {
    #[error("Failed to parse arguments")]
    ParseError(#[from] clap::Error),
}
