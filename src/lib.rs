pub mod controller;
pub mod error;
pub mod model;
pub mod process;
pub mod worker;

#[cfg(test)]
mod tests {
    use crate::model::{Model, Rule};
    use std::fs;

    // Test the creation and retrieval of an item in the model
    #[test]
    fn test_create_and_read_item() {
        let mut model = Model::new();
        let item_name = "TestItem";
        let rules = vec![Rule {
            path: "/path/to/some/process".to_string(),
        }];
        let state = true;
        let cloned_rules = rules.to_vec();
        // Create an item
        model
            .create_item(item_name.to_string(), cloned_rules, state)
            .unwrap();

        // Read the item and check its properties
        let item = model.read_item(item_name).unwrap();
        assert_eq!(item.name, item_name);
        assert_eq!(item.rules.to_vec(), rules.to_vec());
        assert_eq!(item.state, state);
    }

    // Test updating an item in the model
    #[test]
    fn test_update_item() {
        let mut model = Model::new();
        let item_name = "TestItem";
        let rules = vec![Rule {
            path: "/path/to/some/process".to_string(),
        }];
        let state = true;

        // Create an item
        model
            .create_item(item_name.to_string(), rules, state)
            .unwrap();

        // Update the item
        let new_rules = vec![Rule {
            path: "/path/to/another/process".to_string(),
        }];
        let new_state = false;
        model
            .update_item(item_name, new_rules.clone(), new_state)
            .unwrap();

        // Read the updated item and check its properties
        let updated_item = model.read_item(item_name).unwrap();
        assert_eq!(updated_item.name, item_name);
        assert_eq!(updated_item.rules, new_rules);
        assert_eq!(updated_item.state, new_state);
    }

    // Test deleting an item from the model
    #[test]
    fn test_delete_item() {
        let mut model = Model::new();
        let item_name = "TestItem";
        let rules = vec![Rule {
            path: "/path/to/some/process".to_string(),
        }];
        let state = true;

        // Create an item
        model
            .create_item(item_name.to_string(), rules, state)
            .unwrap();

        // Delete the item
        model.delete_item(item_name).unwrap();

        // Try to read the deleted item, it should return None
        assert!(model.read_item(item_name).is_none());
    }

    // Test saving and loading the model
    #[test]
    fn test_save_and_load_model() {
        let mut model = Model::new();
        let item_name = "TestItem";
        let rules = vec![Rule {
            path: "/path/to/some/process".to_string(),
        }];
        let state = true;

        // Create an item
        model
            .create_item(item_name.to_string(), rules.clone(), state)
            .unwrap();

        // Save the model to a file
        model.save().unwrap();

        // Create a new model and load the saved data
        let mut loaded_model = Model::new();
        loaded_model.load().unwrap();

        // Read the item from the loaded model and check its properties
        let loaded_item = loaded_model.read_item(item_name).unwrap();
        assert_eq!(loaded_item.name, item_name);
        assert_eq!(loaded_item.rules, rules);
        assert_eq!(loaded_item.state, state);

        // Clean up: delete the saved file
        fs::remove_file("config.json").unwrap();
    }
}
