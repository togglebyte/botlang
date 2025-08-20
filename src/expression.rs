use crate::op::Op;

#[derive(Debug, Clone)]
pub enum Expression {
    Deferred(String),
    Ident(String),
    Str(String),
    Int(i64),
    Float(f64),
    Binary {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        op: Op,
    },
    Unary {
        value: Box<Expression>,
        op: Op,
    },
    Fn {
        fun: Box<Expression>,
        args: Vec<Expression>,
    },
    Array(Vec<Expression>),
    Index {
        src: Box<Expression>,
        key: Box<Expression>,
    },
    Declaration {
        ident: String,
        value: Box<Expression>,
    },
    Assignment {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    // The bestest expression
    Null,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deferred(_) => panic!("this should never be printed"),
            Self::Ident(i) => write!(f, "{i}"),
            Self::Str(s) => write!(f, "{s}"),
            Self::Binary { lhs, rhs, op } => write!(f, "({op} {lhs} {rhs})"),
            Self::Unary { value, op } => write!(f, "({op}{value})"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(i) => write!(f, "{i}"),
            Self::Fn { fun, args } => {
                write!(f, "<fn ")?;
                write!(f, "{fun}")?;
                write!(f, "(")?;

                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ")")
            }
            Self::Array(values) => {
                write!(f, "[")?;

                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{value}")?;
                }

                write!(f, "]")
            }
            Self::Declaration { ident, value } => write!(f, "let {ident} = {value}"),
            Self::Assignment { lhs, rhs } => write!(f, "{lhs} = {rhs}"),
            Self::Index { src, key } => write!(f, "{src}[{key}]"),
            Self::Null => write!(f, "<null>"),
        }
    }
}
