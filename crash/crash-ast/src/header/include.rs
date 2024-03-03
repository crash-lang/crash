use std::path::Path;

#[derive(Clone)]
pub struct Include {
    path: &'static Path
}

impl Include {
    pub fn new(path: &'static Path) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &'static Path {
        self.path
    }
}