use std::fmt::Write;

use crate::context::Context;
use crate::eval::eval;
pub use crate::expression::Expression;
use crate::fun::Fun;
use crate::lexer::lex;
use crate::pratt::parse;

pub mod context;
pub mod error;
pub mod eval;
pub mod expression;
pub mod fun;
pub mod lexer;
pub mod op;
pub mod pratt;
pub mod vars;

pub struct Interpreter<U> {
    context: Context<U>,
    output: String,
}

impl<U> Interpreter<U>
where
    U: Send + 'static,
{
    pub fn new(user_data: U) -> Self {
        Self {
            context: Context::new(user_data),
            output: String::new(),
        }
    }

    pub fn register_fun(&mut self, name: &'static str, fun: Fun<U>) {
        self.context.functions.register(name, fun);
    }

    pub fn load_read_only(&mut self, values: impl IntoIterator<Item = (String, String)>) {
        self.context.load_read_only(values);
    }

    pub fn run(&mut self, src: &str) -> Option<&str> {
        self.output.clear();
        let tokens = match lex(src) {
            Ok(t) => t,
            Err(e) => {
                _ = write!(&mut self.output, "{e}");
                return Some(&self.output);
            }
        };

        let expr = match parse(tokens) {
            Ok(expr) => expr,
            Err(e) => {
                _ = write!(&mut self.output, "{e}");
                return Some(&self.output);
            }
        };

        let expr = eval(expr, &mut self.context);
        match expr {
            Expression::Null => None,
            expr => {
                _ = write!(&mut self.output, "{expr}");
                Some(&self.output)
            }
        }
    }
}
