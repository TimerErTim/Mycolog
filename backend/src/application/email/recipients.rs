use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Recipient {
    pub email: String,
    pub variables: BTreeMap<String, String>,
}

impl Recipient {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
            variables: BTreeMap::new(),
        }
    }

    pub fn bind(mut self, name: &str, substition: impl ToString) -> Self {
        self.variables
            .insert(name.to_string(), substition.to_string());
        self
    }
}
