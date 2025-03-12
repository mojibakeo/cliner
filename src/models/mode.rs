use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    pub slug: String,
    pub name: String,
    pub role_definition: String,
    pub groups: Vec<String>,
    pub custom_instructions: Option<String>,
}

impl Mode {
    pub fn new(
        slug: String,
        name: String,
        role_definition: String,
        groups: Vec<String>,
        custom_instructions: Option<String>,
    ) -> Self {
        Self {
            slug,
            name,
            role_definition,
            groups,
            custom_instructions,
        }
    }

    pub fn to_json(&self) -> Result<Value> {
        let json_value = serde_json::to_value(self)?;
        Ok(json_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_creation() {
        let mode = Mode::new(
            "test-mode".to_string(),
            "Test Mode".to_string(),
            "Test Role Definition".to_string(),
            vec!["read".to_string(), "edit".to_string()],
            Some("Custom instructions".to_string()),
        );

        assert_eq!(mode.slug, "test-mode");
        assert_eq!(mode.name, "Test Mode");
        assert_eq!(mode.role_definition, "Test Role Definition");
        assert_eq!(mode.groups, vec!["read", "edit"]);
        assert_eq!(mode.custom_instructions, Some("Custom instructions".to_string()));
    }

    #[test]
    fn test_to_json() {
        let mode = Mode::new(
            "test-mode".to_string(),
            "Test Mode".to_string(),
            "Test Role".to_string(),
            vec!["read".to_string()],
            None,
        );

        let json_result = mode.to_json();
        assert!(json_result.is_ok());

        let json_value = json_result.unwrap();
        assert_eq!(json_value["slug"], "test-mode");
        assert_eq!(json_value["name"], "Test Mode");
        assert_eq!(json_value["groups"][0], "read");
        assert_eq!(json_value["role_definition"], "Test Role");
    }
}
