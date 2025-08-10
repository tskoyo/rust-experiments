use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post { title, body }
    }

    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}
