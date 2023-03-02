use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("API error")]
#[diagnostic(code(api::error))]
pub enum ApiError {
    #[error("Failed to make request")]
    RequestError(#[source] reqwest::Error),
}
