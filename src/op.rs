#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Plus,
    Minus,
    Mul,
    Div,
    Equal,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::Equal => write!(f, "="),
        }
    }
}
