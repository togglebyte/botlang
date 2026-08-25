use crate::expression::Expression;

pub(super) fn sin<U>(args: &[Expression], _: &U, _: &str) -> Expression {
    if args.len() != 1 {
        return Expression::Null;
    }

    let num = match &args[0] {
        Expression::Float(n) => *n,
        Expression::Int(i) => *i as f64,
        _ => return Expression::Null,
    };

    Expression::Float(num.sin())
}

pub(super) fn asin<U>(args: &[Expression], _: &U, _: &str) -> Expression {
    if args.len() != 1 {
        return Expression::Null;
    }

    let num = match &args[0] {
        Expression::Float(n) => *n,
        Expression::Int(i) => *i as f64,
        _ => return Expression::Null,
    };

    Expression::Float(num.asin())
}

pub(super) fn cos<U>(args: &[Expression], _: &U, _: &str) -> Expression {
    if args.len() != 1 {
        return Expression::Null;
    }

    let num = match &args[0] {
        Expression::Float(n) => *n,
        Expression::Int(i) => *i as f64,
        _ => return Expression::Null,
    };

    Expression::Float(num.cos())
}

pub(super) fn pow<U>(args: &[Expression], _: &U, _: &str) -> Expression {
    if args.len() != 2 {
        return Expression::Null;
    }

    let lhs = match &args[0] {
        Expression::Float(n) => *n,
        Expression::Int(i) => *i as f64,
        _ => return Expression::Null,
    };

    let rhs = match &args[1] {
        Expression::Float(n) => *n,
        Expression::Int(i) => *i as f64,
        _ => return Expression::Null,
    };

    Expression::Float(lhs.powf(rhs))
}
