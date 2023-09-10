use crate::error::ProjectError;
use crate::model::{Item, Model, Rule}; // Make sure to adjust the import path to match your module structure

#[allow(unused)]
pub(crate) struct Controller {
    model: Model,
}

impl Controller {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    pub fn create_item(
        &mut self,
        name: &str,
        state: bool,
        rules: Vec<Rule>,
    ) -> Result<(), ProjectError> {
        self.model
            .create_item(name.parse().unwrap_or("".to_string()), rules, state)
    }

    pub(crate) fn read_item(&self, name: &str) -> Option<&Item> {
        self.model.read_item(name)
    }

    pub fn update_item(
        &mut self,
        name: &str,
        new_state: bool,
        new_rules: Vec<Rule>,
    ) -> Result<(), ProjectError> {
        self.model.update_item(name, new_rules, new_state)
    }

    pub fn delete_item(&mut self, name: &str) -> Result<(), ProjectError> {
        self.model.delete_item(name)
    }

    pub fn update_rules(
        &mut self,
        item_name: &str,
        new_rules: Vec<Rule>,
    ) -> Result<(), ProjectError> {
        self.model.update_item(item_name, new_rules, true)
    }

    pub fn get_all_items(&self) -> &[Item] {
        &self.model.items
    }
}
