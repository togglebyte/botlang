use crate::context::Context;
use crate::expression::Expression;
use crate::lexer::lex;
use crate::op::Op;
use crate::pratt::parse;

pub fn eval<U>(expression: Expression, ctx: &mut Context<U>) -> Expression
where
    U: Send + 'static,
{
    match expression {
        Expression::Deferred(src) => {
            let Ok(tokens) = lex(&src) else { return Expression::Null };
            let Ok(expr) = parse(tokens) else { return Expression::Null };
            eval(expr, ctx)
        }
        Expression::Ident(ident) => eval(ctx.variables.get(&ident), ctx),
        Expression::Str(_) | Expression::Int(_) | Expression::Float(_) | Expression::Array(_) => expression,
        Expression::Binary { lhs, rhs, op } => eval_op(*lhs, *rhs, op, ctx),
        Expression::Unary { value, op } => match op {
            Op::Minus => match *value {
                Expression::Int(i) => Expression::Int(-i),
                _ => Expression::Null,
            },
            _ => Expression::Null,
        },
        Expression::Fn { fun, args } => match *fun {
            Expression::Ident(key) => {
                let args = args.into_iter().map(|arg| eval(arg, ctx)).collect();
                ctx.call(&key, args)
            }
            _ => Expression::Null,
        },
        Expression::Declaration { ident, value } => {
            let value = eval(*value, ctx);
            if let Expression::Null = value {
                ctx.variables.remove(&ident);
                return Expression::Null;
            }

            ctx.variables.push(ident, value);
            Expression::Null
        }
        Expression::Assignment { mut lhs, rhs } => {
            let rhs = eval(*rhs, ctx);

            if let Some(lhs) = assign_eval_mut(&mut lhs, ctx) {
                *lhs = rhs;
            }
            Expression::Null
        }
        Expression::Index { src, key } => eval_index(*src, *key, ctx),
        Expression::Null => Expression::Null,
    }
}

fn assign_eval_mut<'a, U>(expr: &mut Expression, ctx: &'a mut Context<U>) -> Option<&'a mut Expression>
where
    U: Send + 'static,
{
    match expr {
        Expression::Ident(name) => ctx.variables.get_mut(name),
        Expression::Index { src, key } => {
            let Expression::Int(key) = eval(*key.clone(), ctx) else { return None };
            let src = assign_eval_mut(src, ctx)?;
            match src {
                Expression::Array(expressions) => expressions.get_mut(key as usize),
                _ => None,
            }
        }
        _ => None,
    }
}

fn eval_index<U>(src: Expression, key: Expression, ctx: &mut Context<U>) -> Expression
where
    U: Send + 'static,
{
    let src = eval(src, ctx);
    match src {
        Expression::Array(mut expressions) if !expressions.is_empty() => {
            let Expression::Int(key) = eval(key, ctx) else { return Expression::Null };
            if key as usize >= expressions.len() {
                return Expression::Null;
            }

            let expression = expressions.remove(key as usize);
            eval(expression, ctx)
        }
        Expression::Str(_) => Expression::Null,
        Expression::Array(_)
        | Expression::Deferred(_)
        | Expression::Ident(_)
        | Expression::Int(_)
        | Expression::Float(_)
        | Expression::Binary { .. }
        | Expression::Unary { .. }
        | Expression::Assignment { .. }
        | Expression::Declaration { .. }
        | Expression::Fn { .. }
        | Expression::Null => Expression::Null,
        Expression::Index { .. } => {
            let src = eval(src, ctx);
            eval_index(src, key, ctx)
        }
    }
}

fn eval_op<U>(lhs: Expression, rhs: Expression, op: Op, ctx: &mut Context<U>) -> Expression
where
    U: Send + 'static,
{
    match (eval(lhs, ctx), eval(rhs, ctx)) {
        (Expression::Str(lhs), Expression::Str(rhs)) => Expression::Str(format!("{lhs}{rhs}")),
        (Expression::Int(lhs), Expression::Int(rhs)) => int_op(lhs, rhs, op),
        (Expression::Int(lhs), Expression::Float(rhs)) => float_op(lhs as f64, rhs, op),
        (Expression::Float(lhs), Expression::Int(rhs)) => float_op(lhs, rhs as f64, op),
        (Expression::Float(lhs), Expression::Float(rhs)) => float_op(lhs, rhs, op),
        _ => Expression::Null,
    }
}

fn int_op(lhs: i64, rhs: i64, op: Op) -> Expression {
    match op {
        Op::Plus => Expression::Int(lhs + rhs),
        Op::Minus => Expression::Int(lhs - rhs),
        Op::Mul => Expression::Int(lhs * rhs),
        Op::Div if rhs != 0 => Expression::Int(lhs / rhs),
        _ => Expression::Null,
    }
}

fn float_op(lhs: f64, rhs: f64, op: Op) -> Expression {
    match op {
        Op::Plus => Expression::Float(lhs + rhs),
        Op::Minus => Expression::Float(lhs - rhs),
        Op::Mul => Expression::Float(lhs * rhs),
        Op::Div if rhs != 0.0 => Expression::Float(lhs / rhs),
        _ => Expression::Null,
    }
}
