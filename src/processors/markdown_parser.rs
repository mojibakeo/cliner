use serde_json::{json, Value, Map};
use crate::error::{ClinerError, Result};
use crate::models::Mode;

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse_to_mode(markdown_content: &str) -> Result<Mode> {
        let sections = markdown_content.split("---").collect::<Vec<&str>>();
        
        Self::validate_has_sections(&sections)?;
        
        let header_lines = Self::extract_header_lines(&sections)?;
        let role_content = Self::extract_role_content(&sections)?;
        let mode_name = Self::extract_mode_name(&header_lines)?;
        let groups = Self::extract_groups(&header_lines);
        let description = Self::extract_field_value(&header_lines, "description:");
        let restrictions = Self::extract_restrictions(&header_lines);
        let default_output = Self::extract_field_value(&header_lines, "default_output:");
        
        Ok(Mode::new(
            mode_name,
            groups,
            role_content,
            description,
            restrictions,
            default_output
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

    fn extract_mode_name(lines: &[String]) -> Result<String> {
        let mode_field_name = "mode_name:";
        let mode_name = Self::extract_field_value(lines, mode_field_name);
        
        match mode_name {
            Some(name) => Ok(name),
            None => Err(ClinerError::MissingField("Missing 'mode_name:' field in Markdown".to_string())),
        }
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

    fn extract_restrictions(lines: &[String]) -> Option<Value> {
        let section_marker = "restrictions:";
        let mut restriction_map = Map::new();
        let mut currently_in_section = false;
        
        for line in lines {
            let trimmed_line = line.trim();
            
            if trimmed_line == section_marker {
                currently_in_section = true;
                continue;
            }
            
            if !currently_in_section {
                continue;
            }
            
            if line.starts_with("- ") {
                let item_text = line.replace("- ", "").trim().to_string();
                let key_value_parts: Vec<&str> = item_text.split(": ").collect();
                
                if key_value_parts.len() == 2 {
                    let key = key_value_parts[0].to_string();
                    let value = key_value_parts[1];
                    restriction_map.insert(key, json!(value));
                }
                
                continue;
            }
            
            if !trimmed_line.is_empty() {
                break;
            }
        }
        
        if restriction_map.is_empty() {
            return None;
        }
        
        Some(json!(restriction_map))
    }
}

pub fn markdown_to_json(markdown_content: &str) -> Result<Value> {
    MarkdownParser::parse_to_json(markdown_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_markdown() -> String {
        "mode_name: テストモード
groups:
- read
- write
description: テストモードの説明
restrictions:
- fileRegex: \\.rs$

---

# テストロール定義

これはテストロールの詳細な説明なのだ。".to_string()
    }

    #[test]
    fn test_parse_to_mode() {
        let markdown = create_test_markdown();
        let mode_result = MarkdownParser::parse_to_mode(&markdown);
        
        assert!(mode_result.is_ok());
        let mode = mode_result.unwrap();
        
        assert_eq!(mode.mode_name, "テストモード");
        assert_eq!(mode.groups, vec!["read", "write"]);
        assert_eq!(mode.description, Some("テストモードの説明".to_string()));
        assert!(mode.role.contains("テストロール定義"));
    }

    #[test]
    fn test_extract_mode_name() {
        let lines = vec![
            "mode_name: テストモード".to_string(),
            "groups:".to_string(),
        ];
        
        let mode_name = MarkdownParser::extract_mode_name(&lines);
        assert!(mode_name.is_ok());
        assert_eq!(mode_name.unwrap(), "テストモード");
    }

    #[test]
    fn test_missing_mode_name() {
        let lines = vec![
            "description: テスト".to_string(),
        ];
        
        let mode_name = MarkdownParser::extract_mode_name(&lines);
        assert!(mode_name.is_err());
    }

    #[test]
    fn test_extract_groups() {
        let lines = vec![
            "mode_name: テスト".to_string(),
            "groups:".to_string(),
            "- read".to_string(),
            "- write".to_string(),
            "description: テスト".to_string(),
        ];
        
        let groups = MarkdownParser::extract_groups(&lines);
        assert_eq!(groups, vec!["read", "write"]);
    }

    #[test]
    fn test_extract_restrictions() {
        let lines = vec![
            "restrictions:".to_string(),
            "- fileRegex: \\.rs$".to_string(),
            "- otherRestriction: value".to_string(),
            "".to_string(),
            "description: テスト".to_string(),
        ];
        
        let restrictions = MarkdownParser::extract_restrictions(&lines);
        assert!(restrictions.is_some());
        
        let json_value = restrictions.unwrap();
        assert_eq!(json_value["fileRegex"], "\\.rs$");
        assert_eq!(json_value["otherRestriction"], "value");
    }

    #[test]
    fn test_extract_field_value() {
        let lines = vec![
            "mode_name: テストモード".to_string(),
            "description: テスト説明".to_string(),
        ];
        
        let description = MarkdownParser::extract_field_value(&lines, "description:");
        assert_eq!(description, Some("テスト説明".to_string()));
        
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
        
        assert_eq!(json["mode_name"], "テストモード");
        assert!(json["role"].as_str().unwrap().contains("テストロール定義"));
    }
}
