use std::collections::HashMap;

use crate::expression::Expression;

mod maths;
mod string;

pub type Fun<U> = Box<dyn Fn(&[Expression], &U, &str) -> Expression + Send>;

pub struct Functions<U> {
    inner: HashMap<&'static str, Fun<U>>,
}

impl<U> Functions<U>
where
    U: Send + 'static,
{
    pub fn new() -> Self {
        let mut inner: HashMap<&'static str, Fun<U>> = HashMap::new();
        inner.insert("to_lower", Box::new(string::to_lower));
        inner.insert("to_upper", Box::new(string::to_upper));
        inner.insert("cos", Box::new(maths::cos));
        inner.insert("sin", Box::new(maths::sin));
        inner.insert("asin", Box::new(maths::asin));
        inner.insert("pow", Box::new(maths::pow));
        Self { inner }
    }

    pub fn register(&mut self, name: &'static str, fun: Fun<U>) {
        self.inner.insert(name, fun);
    }

    pub(crate) fn call(&self, fun: &str, args: Vec<Expression>, user_data: &U, caller: &str) -> Expression {
        let Some(fun) = self.inner.get(fun) else { return Expression::Null };
        fun(&args, user_data, caller)
    }
}
