pub enum Endpoint {
    Actions,
    Consignment,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Actions => write!(f, "actions"),
            Self::Consignment => write!(f, "consignment"),
        }
    }
}
