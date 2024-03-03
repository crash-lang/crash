use std::path::Path;

#[derive(Clone)]
pub struct Include {
    path: String
}

impl Include {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}