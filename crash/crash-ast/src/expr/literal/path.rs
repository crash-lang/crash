
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathLiteralExpr {
    path: String
}

impl PathLiteralExpr {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    
    pub fn path(self) -> String {
        self.path
    }
}
