use crate::api::client::AocApi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub year: u16,
    pub day: u8,
    pub session_id: String,
}

impl Config {
    pub fn get_client(&self) -> AocApi {
        AocApi::new(self.session_id.clone())
    }
}
