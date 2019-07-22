use crate::entity::form::{
    settings::{Field, ValidForm as EntityValidForm},
    FormField,
};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ValidForm<'a> {
    user: IndexMap<&'a str, &'a str>,
}

impl<'a> ValidForm<'a> {
    pub fn new(form: &'a EntityValidForm) -> Self {
        ValidForm {
            user: form
                .iter()
                .filter_map(|(key, field)| match field {
                    Field::Password(password) if password.is_empty() => None,
                    _ => Some((*key, field.value())),
                })
                .collect(),
        }
    }
}
