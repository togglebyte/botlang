use crate::expression::Expression;

pub(super) fn to_lower<U>(args: &[Expression], _: &U, _: &str) -> Expression {
    if args.len() != 1 {
        return Expression::Null;
    }

    match &args[0] {
        Expression::Str(s) => Expression::Str(s.to_ascii_lowercase().to_string()),
        _ => Expression::Null,
    }
}

pub(super) fn to_upper<U>(args: &[Expression], _: &U, _: &str) -> Expression {
    if args.len() != 1 {
        return Expression::Null;
    }

    match &args[0] {
        Expression::Str(s) => Expression::Str(s.to_ascii_uppercase().to_string()),
        _ => Expression::Null,
    }
}
