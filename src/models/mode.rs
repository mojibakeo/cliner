use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    pub mode_name: String,
    pub groups: Vec<String>,
    pub role: String,
    pub description: Option<String>,
    pub restrictions: Option<Value>,
    pub default_output: Option<String>,
}

impl Mode {
    pub fn new(
        mode_name: String,
        groups: Vec<String>,
        role: String,
        description: Option<String>,
        restrictions: Option<Value>,
        default_output: Option<String>,
    ) -> Self {
        Self {
            mode_name,
            groups,
            role,
            description,
            restrictions,
            default_output,
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
    use serde_json::json;

    #[test]
    fn test_mode_creation() {
        let mode = Mode::new(
            "Test Mode".to_string(),
            vec!["read".to_string(), "write".to_string()],
            "Test Role Definition".to_string(),
            Some("Test Description".to_string()),
            Some(json!({"fileRegex": "\\.rs$"})),
            Some("Default output".to_string()),
        );

        assert_eq!(mode.mode_name, "Test Mode");
        assert_eq!(mode.groups, vec!["read", "write"]);
        assert_eq!(mode.role, "Test Role Definition");
        assert_eq!(mode.description, Some("Test Description".to_string()));
        assert!(mode.restrictions.is_some());
        assert_eq!(mode.default_output, Some("Default output".to_string()));
    }

    #[test]
    fn test_to_json() {
        let mode = Mode::new(
            "Test Mode".to_string(),
            vec!["read".to_string()],
            "Test Role".to_string(),
            None,
            None,
            None,
        );

        let json_result = mode.to_json();
        assert!(json_result.is_ok());

        let json_value = json_result.unwrap();
        assert_eq!(json_value["mode_name"], "Test Mode");
        assert_eq!(json_value["groups"][0], "read");
        assert_eq!(json_value["role"], "Test Role");
    }
}
