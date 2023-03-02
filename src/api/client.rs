use miette::{IntoDiagnostic, Result};
use reqwest::header::COOKIE;

pub struct AocApi {
    pub client: reqwest::Client,
    pub session_id: String,
}

impl AocApi {
    pub fn new(session_id: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .user_agent("aoc-cli")
            .build()
            .expect("Failed to build reqwest client");
        Self { client, session_id }
    }

    pub async fn get_problem_input(&self, year: u16, day: u8) -> Result<String> {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let response = self
            .client
            .get(&url)
            .header(COOKIE, format!("session={}", self.session_id))
            .send()
            .await
            .into_diagnostic()?;
        let input = response.text().await.into_diagnostic()?;
        Ok(input)
    }
}
