use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bang {
    pub alias: String,
    pub url: String,
}

impl Bang {
    pub fn new(alias: String, url: String) -> Self {
        Bang { alias, url }
    }

    pub fn pretty_print(&self) {
        println!("{} -> {}", self.alias, self.url)
    }
}
