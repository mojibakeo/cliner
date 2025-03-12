use serde_json::Value;
use crate::error::{ClinerError, Result};
use crate::models::Mode;

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse_to_mode(markdown_content: &str) -> Result<Mode> {
        let sections = markdown_content.split("---").collect::<Vec<&str>>();
        
        Self::validate_has_sections(&sections)?;
        
        let header_lines = Self::extract_header_lines(&sections)?;
        let role_definition = Self::extract_role_content(&sections)?;
        let mode_name = Self::extract_mode_name(&header_lines)?;
        
        let slug = Self::extract_slug(&header_lines).unwrap_or_else(|| {
            Self::extract_slug_from_mode_name(&mode_name)
        });
        
        let groups = Self::extract_groups(&header_lines);
        let custom_instructions = Self::extract_field_value(&header_lines, "customInstructions:");
        
        Ok(Mode::new(
            slug,
            mode_name,
            role_definition,
            groups,
            custom_instructions
        ))
    }
    
    pub fn parse_to_json(markdown_content: &str) -> Result<Value> {
        let mode = Self::parse_to_mode(markdown_content)?;
        mode.to_json()
    }

    fn validate_has_sections(sections: &[&str]) -> Result<()> {
        if sections.len() < 2 {
            return Err(ClinerError::InvalidFormat("Missing '---' separator in Markdown".to_string()));
        }
        Ok(())
    }

    fn extract_header_lines(sections: &[&str]) -> Result<Vec<String>> {
        Self::validate_has_sections(sections)?;
        let lines = sections[0].lines().map(String::from).collect();
        Ok(lines)
    }

    fn extract_role_content(sections: &[&str]) -> Result<String> {
        Self::validate_has_sections(sections)?;
        let content = sections[1..].join("---").trim().to_string();
        Ok(content)
    }

    fn extract_slug_from_mode_name(mode_name: &str) -> String {
        let sanitized = mode_name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
            .collect::<String>();
        
        let mut result = String::new();
        let mut last_was_hyphen = false;
        
        for c in sanitized.chars() {
            if c == '-' {
                if !last_was_hyphen {
                    result.push(c);
                }
                last_was_hyphen = true;
            } else {
                result.push(c);
                last_was_hyphen = false;
            }
        }
        
        result.trim_matches('-').to_string()
    }

    fn extract_mode_name(lines: &[String]) -> Result<String> {
        let name_field = Self::extract_field_value(lines, "name:");
        if let Some(name) = name_field {
            return Ok(name);
        }
        
        let mode_field_name = "mode_name:";
        let mode_name = Self::extract_field_value(lines, mode_field_name);
        
        match mode_name {
            Some(name) => Ok(name),
            None => Err(ClinerError::MissingField("Missing 'name:' or 'mode_name:' field in Markdown".to_string())),
        }
    }

    fn extract_slug(lines: &[String]) -> Option<String> {
        Self::extract_field_value(lines, "slug:")
    }

    fn extract_field_value(lines: &[String], field_prefix: &str) -> Option<String> {
        lines.iter()
             .find(|line| line.starts_with(field_prefix))
             .map(|line| line.replace(field_prefix, "").trim().to_string())
    }

    fn extract_groups(lines: &[String]) -> Vec<String> {
        let groups_section_marker = "groups:";
        let mut group_items = Vec::new();
        let mut currently_in_section = false;
        
        for line in lines {
            let trimmed_line = line.trim();
            
            if trimmed_line == groups_section_marker {
                currently_in_section = true;
                continue;
            }
            
            if !currently_in_section {
                continue;
            }
            
            if line.starts_with("- ") {
                let group_name = line.replace("- ", "").trim().to_string();
                group_items.push(group_name);
                continue;
            }
            
            if !trimmed_line.is_empty() {
                break;
            }
        }
        
        group_items
    }

}

pub fn markdown_to_json(markdown_content: &str) -> Result<Value> {
    MarkdownParser::parse_to_json(markdown_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_markdown() -> String {
        "name: TestMode
slug: test-mode
groups:
- read
- edit 
customInstructions: Test mode description

---

# Test Role Definition

This is a detailed description of the test role.".to_string()
    }

    #[test]
    fn test_parse_to_mode() {
        let markdown = create_test_markdown();
        let mode_result = MarkdownParser::parse_to_mode(&markdown);
        
        assert!(mode_result.is_ok());
        let mode = mode_result.unwrap();
        
        assert_eq!(mode.name, "TestMode");
        assert_eq!(mode.slug, "test-mode");
        assert_eq!(mode.groups, vec!["read", "edit"]);
        assert!(mode.role_definition.contains("Test Role Definition"));
    }

    #[test]
    fn test_extract_mode_name() {
        let lines = vec![
            "mode_name: TestMode".to_string(),
            "groups:".to_string(),
        ];
        
        let mode_name = MarkdownParser::extract_mode_name(&lines);
        assert!(mode_name.is_ok());
        assert_eq!(mode_name.unwrap(), "TestMode");
    }

    #[test]
    fn test_missing_mode_name() {
        let lines = vec![
            "description: Test".to_string(),
        ];
        
        let mode_name = MarkdownParser::extract_mode_name(&lines);
        assert!(mode_name.is_err());
    }
    
    #[test]
    fn test_extract_slug() {
        let lines = vec![
            "name: TestMode".to_string(),
            "slug: explicit-slug-with-hyphens".to_string(),
        ];
        
        let slug = MarkdownParser::extract_slug(&lines);
        assert_eq!(slug, Some("explicit-slug-with-hyphens".to_string()));
    }
    
    #[test]
    fn test_slug_preference() {
        let markdown = "name: Test Mode
slug: my-explicit-slug
groups:
- read
- edit

---

# Test Role Definition";
        
        let mode = MarkdownParser::parse_to_mode(markdown).unwrap();
        assert_eq!(mode.slug, "my-explicit-slug");
        
        let markdown_without_slug = "name: Test Mode
groups:
- read
- edit

---

# Test Role Definition";
        
        let mode = MarkdownParser::parse_to_mode(markdown_without_slug).unwrap();
        assert_eq!(mode.slug, "test-mode");
    }

    #[test]
    fn test_extract_groups() {
        let lines = vec![
            "mode_name: Test".to_string(),
            "groups:".to_string(),
            "- read".to_string(),
            "- edit".to_string(),
            "description: Test".to_string(),
        ];
        
        let groups = MarkdownParser::extract_groups(&lines);
        assert_eq!(groups, vec!["read", "edit"]);
    }

    #[test]
    fn test_extract_field_value() {
        let lines = vec![
            "mode_name: TestMode".to_string(),
            "description: Test description".to_string(),
        ];
        
        let description = MarkdownParser::extract_field_value(&lines, "description:");
        assert_eq!(description, Some("Test description".to_string()));
        
        let nonexistent = MarkdownParser::extract_field_value(&lines, "nonexistent:");
        assert_eq!(nonexistent, None);
    }

    #[test]
    fn test_validate_has_sections() {
        let valid_sections = vec!["header", "content"];
        assert!(MarkdownParser::validate_has_sections(&valid_sections).is_ok());
        
        let invalid_sections = vec!["header"];
        assert!(MarkdownParser::validate_has_sections(&invalid_sections).is_err());
    }

    #[test]
    fn test_markdown_to_json() {
        let markdown = create_test_markdown();
        let json_result = markdown_to_json(&markdown);
        
        assert!(json_result.is_ok());
        let json = json_result.unwrap();
        
        assert_eq!(json["name"], "TestMode");
        assert_eq!(json["slug"], "test-mode");
        assert!(json["role_definition"].as_str().unwrap().contains("Test Role Definition"));
    }
}
