use crate::api::client::AocApi;
use crate::config::Config;
use std::path::PathBuf;

pub struct AppContext {
    pub config: Config,
    pub api: AocApi,
    pub problem_input: Option<String>,
    pub project_dir: Option<PathBuf>,
}

impl AppContext {
    pub fn new(config: Config) -> Self {
        let api = config.get_client();
        Self {
            config,
            api,
            problem_input: None,
            project_dir: None,
        }
    }
}
