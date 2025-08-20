use crate::error::{Error, Result};
use crate::expression::Expression;
use crate::lexer::{Token, Tokens};
use crate::op::Op;

fn infix_bp(op: Op) -> Option<(u8, u8)> {
    let power = match op {
        Op::Plus | Op::Minus => (1, 2),
        Op::Mul | Op::Div => (3, 4),
        Op::LParen => (3, 4),
        _ => return None,
    };

    Some(power)
}

fn postfix_bp(op: Op) -> Option<u8> {
    let power = match op {
        Op::LParen | Op::LBracket | Op::Equal => 5,
        _ => return None,
    };

    Some(power)
}

fn prefix_bp(op: Op) -> Option<u8> {
    match op {
        Op::Plus | Op::Minus => Some(5),
        _ => return None,
    }
}

pub fn parse(mut tokens: Tokens) -> Result<Expression> {
    parse_with(&mut tokens, 0)
}

fn parse_with(tokens: &mut Tokens, bp: u8) -> Result<Expression> {
    let mut lhs = match tokens.peek() {
        &Token::Op(Op::LParen) => {
            tokens.consume();
            let lhs = parse_with(tokens, 0)?;
            // dispose and assure that the next token is the closing bracket
            if tokens.peek() != &Token::Op(Op::RParen) {
                return Err(Error::MissingClosingParen);
            }
            tokens.consume();
            lhs
        }
        &Token::Op(Op::LBracket) => {
            tokens.consume();
            parse_array(tokens)?
        }

        &Token::Op(op) => {
            let Some(r) = prefix_bp(op) else { return Err(Error::InvalidOperator(op)) };
            tokens.consume();
            let value = parse_with(tokens, r)?;
            return Ok(Expression::Unary {
                value: Box::new(value),
                op,
            });
        }
        Token::Let => {
            tokens.consume();
            return Ok(parse_decl(tokens)?);
        }
        Token::Ident(_) => Expression::Ident(tokens.take_string()),
        Token::Str(_) => Expression::Str(tokens.take_string()),
        Token::Int(_) => Expression::Int(tokens.take_int()),
        Token::Float(_) => Expression::Float(tokens.take_float()),
        token => return Err(Error::NotEnoughCake(token.clone())),
    };

    loop {
        let op = match tokens.peek() {
            &Token::Op(op) => op,
            Token::Eof => break,
            _ => break,
        };

        if let Some(l) = postfix_bp(op) {
            if l < bp {
                break;
            }

            tokens.consume();

            // Assignment
            if let Op::Equal = op {
                lhs = Expression::Assignment {
                    lhs: lhs.into(),
                    rhs: parse_with(tokens, l)?.into(),
                };
            }

            // Index
            if let Op::LBracket = op {
                lhs = Expression::Index {
                    src: lhs.into(),
                    key: parse_with(tokens, l)?.into(),
                };

                // consume right bracket
                match tokens.take() {
                    Token::Op(Op::RBracket) => continue,
                    token => return Err(Error::InvalidToken(token)),
                }
            }

            // Function call
            if let Op::LParen = op {
                // method(value, 123)
                lhs = Expression::Fn {
                    fun: Box::new(lhs),
                    args: parse_args(tokens)?,
                };
            }

            continue;
        }

        if let Some((l, r)) = infix_bp(op) {
            if l < bp {
                break;
            }
            tokens.consume();

            let rhs = parse_with(tokens, r)?;
            lhs = Expression::Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            };
            continue;
        }

        break;
    }

    //  f     .    g     .    h
    //  f.g.h -> f["g"]["h"]
    //  f.g
    //  index(index(ident(f), ident(g)), ident(h))

    Ok(lhs)
}

fn parse_decl(tokens: &mut Tokens) -> Result<Expression> {
    let ident = match tokens.take() {
        Token::Ident(ident) => ident,
        token => return Err(Error::InvalidToken(token)),
    };

    match tokens.take() {
        Token::Op(Op::Equal) => (),
        token => return Err(Error::InvalidToken(token)),
    }

    let value = parse_with(tokens, 0)?;

    Ok(Expression::Declaration {
        ident,
        value: Box::new(value),
    })
}

fn parse_args(tokens: &mut Tokens) -> Result<Vec<Expression>> {
    let mut values = vec![];

    loop {
        if let Token::Op(Op::RParen) = tokens.peek() {
            tokens.consume();
            break;
        }

        let value = parse_with(tokens, 0)?;
        values.push(value);

        if let Token::Op(Op::RParen) = tokens.peek() {
            tokens.consume();
            break;
        }

        if let Token::Comma = tokens.peek() {
            tokens.consume();
            continue;
        }

        return Err(Error::MissingComma);
    }

    Ok(values)
}

fn parse_array(tokens: &mut Tokens) -> Result<Expression> {
    let mut values = vec![];

    loop {
        if let Token::Op(Op::RBracket) = tokens.peek() {
            tokens.consume();
            break;
        }

        let value = parse_with(tokens, 0)?;
        values.push(value);

        if let Token::Op(Op::RBracket) = tokens.peek() {
            tokens.consume();
            break;
        }

        if let Token::Comma = tokens.peek() {
            tokens.consume();
            continue;
        }

        return Err(Error::MissingComma);
    }

    Ok(Expression::Array(values))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn addition() {
        // let mut tokens = lex("\"a\" .. \"b\"").unwrap();
        let mut tokens = lex("2 * (1 + 2)").unwrap();
        let mut tokens = lex("2 * 1 + 2").unwrap();
        // let mut tokens = lex("-1").unwrap();
        let expr = parse(tokens).unwrap();
        panic!("{expr}");
    }

    #[test]
    fn fun_fun() {
        // let mut tokens = lex("\"a\" .. \"b\"").unwrap();
        let mut tokens = lex("cos(1) * sin(1)").unwrap();
        // let mut tokens = lex("-1").unwrap();
        let expr = parse(tokens).unwrap();
        panic!("{expr}");
    }
}
