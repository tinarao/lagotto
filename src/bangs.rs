use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bang {
    pub alias: String,
    pub url: String,
}

// bangs are saved in JSON format

impl Bang {
    pub fn new(alias: String, url: String) -> Self {
        Bang { alias, url }
    }
}
