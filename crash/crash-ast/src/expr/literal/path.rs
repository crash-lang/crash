use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathLiteralExpr {
    path: &'static Path
}

impl PathLiteralExpr {
    pub fn new(path: &'static Path) -> Self {
        Self { path }
    }
    
    pub fn path(&self) -> &'static Path {
        self.path
    }
}
