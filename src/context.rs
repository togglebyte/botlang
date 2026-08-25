use crate::fun::Functions;
use crate::vars::Variables;

pub struct Context<U> {
    pub variables: Variables,
    pub functions: Functions<U>,
    pub user_data: U,
}

impl<U> Context<U>
where
    U: Send + 'static,
{
    pub fn new(user_data: U) -> Self {
        Self {
            variables: Variables::empty(),
            functions: Functions::new(),
            user_data,
        }
    }

    pub(crate) fn load_read_only(&mut self, values: impl IntoIterator<Item = (String, String)>) {
        values
            .into_iter()
            .for_each(|(key, value)| self.variables.insert_ro(key, value));
    }

    pub(crate) fn call(&self, key: &str, args: Vec<crate::Expression>, caller: &str) -> crate::Expression {
        self.functions.call(key, args, &self.user_data, caller)
    }
}
