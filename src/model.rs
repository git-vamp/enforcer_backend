use crate::error::ProjectError;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Rule {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleSet {
    pub blocked_items: Vec<Rule>,
}

impl RuleSet {
    pub fn new() -> Self {
        RuleSet {
            blocked_items: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) -> &mut Self {
        self.blocked_items.push(rule);
        self
    }

    pub fn build(&mut self) -> Vec<Rule> {
        std::mem::take(&mut self.blocked_items)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub rules: Vec<Rule>,
    pub state: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub items: Vec<Item>,
}

impl Model {
    pub fn new() -> Self {
        Model { items: Vec::new() }
    }

    // Reimplement the load and save methods for ModelStruct
    pub fn save(&self) -> Result<(), ProjectError> {
        if let Ok(json_data) = serde_json::to_string_pretty(self) {
            if fs::write("config.json", json_data).is_err() {
                return Err(ProjectError::ConfigWriteError);
            }
        } else {
            return Err(ProjectError::ConfigParseError);
        }
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), ProjectError> {
        if let Ok(data) = fs::read_to_string("config.json") {
            if let Ok(parsed_data) = serde_json::from_str::<Model>(&data) {
                self.items = parsed_data.items;
            } else {
                return Err(ProjectError::ConfigParseError);
            }
        } else {
            return Err(ProjectError::ConfigReadError);
        }
        // If there was an error or the data couldn't be parsed, return a default instance.
        Ok(())
    }

    pub fn delete_item(&mut self, item_name: &str) -> Result<(), ProjectError> {
        if let Some(index) = self.items.iter().position(|item| item.name == item_name) {
            self.items.remove(index);
            self.save()?;
        }
        Ok(())
    }
    pub fn update_item(
        &mut self,
        item_name: &str,
        new_rules: Vec<Rule>,
        new_state: bool,
    ) -> Result<(), ProjectError> {
        if let Some(item) = self.items.iter_mut().find(|item| item.name == item_name) {
            item.rules = new_rules;
            item.state = new_state;
            self.save()?;

            // true // Item updated successfully
        }
        Ok(())
    }
    pub fn read_item(&self, item_name: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.name == item_name)
    }

    pub fn create_item(
        &mut self,
        name: String,
        rules: Vec<Rule>,
        state: bool,
    ) -> Result<(), ProjectError> {
        let item = Item { name, rules, state };
        self.items.push(item);
        self.save()
    }
}
