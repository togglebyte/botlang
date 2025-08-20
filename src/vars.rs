use std::collections::HashMap;

use crate::expression::Expression;

pub enum Variable {
    ReadOnly(Expression),
    ReadWrite(Expression),
}

impl Variable {
    fn expr(&self) -> Expression {
        match self {
            Variable::ReadOnly(expr) | Variable::ReadWrite(expr) => expr.clone(),
        }
    }
}

pub struct Variables {
    inner: HashMap<String, Variable>,
}

impl Variables {
    pub fn empty() -> Self {
        Self { inner: HashMap::new() }
    }

    fn can_write(&self, key: &str) -> bool {
        match self.inner.get(key) {
            Some(Variable::ReadOnly(_)) => false,
            _ => true,
        }
    }

    pub(crate) fn get(&self, ident: &str) -> Expression {
        self.inner.get(ident).map(|var| var.expr()).unwrap_or(Expression::Null)
    }

    pub(crate) fn get_mut(&mut self, ident: &str) -> Option<&mut Expression> {
        match self.inner.get_mut(ident) {
            Some(Variable::ReadWrite(expr)) => Some(expr),
            Some(_) | None => None,
        }
    }

    pub(crate) fn push(&mut self, ident: String, value: Expression) {
        if !self.can_write(&ident) {
            return;
        }
        let value = Variable::ReadWrite(value);
        self.inner.insert(ident, value);
    }

    pub(crate) fn remove(&mut self, ident: &str) {
        if !self.can_write(ident) {
            return;
        }
        self.inner.remove(ident);
    }

    pub(crate) fn insert_ro(&mut self, key: String, value: String) {
        self.inner.insert(key, Variable::ReadOnly(Expression::Deferred(value)));
    }
}
