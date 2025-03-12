use std::fs;
use serde_json::Value;
use crate::error::Result;

pub struct OutputGenerator;

impl OutputGenerator {
    pub fn write_json_if_not_empty(json_values: Vec<Value>, output_path: &str, success_message: &str) -> Result<()> {
        if json_values.is_empty() {
            println!("No valid modes found, skipping {} generation", output_path);
            return Ok(());
        }
        
        let custom_modes_obj = serde_json::json!({
            "customModes": json_values
        });
        
        let formatted_json = serde_json::to_string_pretty(&custom_modes_obj)?;
        fs::write(output_path, formatted_json)?;
        println!("{}", success_message);
        Ok(())
    }
    
    pub fn write_content_if_not_empty(content: String, output_path: &str, success_message: &str) -> Result<()> {
        if content.is_empty() {
            println!("No rules found, skipping {} generation", output_path);
            return Ok(());
        }
        
        fs::write(output_path, content)?;
        println!("{}", success_message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;
    use std::path::Path;
    use std::io::Read;

    #[test]
    fn test_write_json_if_not_empty_with_content() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_output.json");
        let output_path_str = output_path.to_str().unwrap();
        
        let json_values = vec![
            json!({"name": "Test1", "value": 123}),
            json!({"name": "Test2", "value": 456})
        ];
        
        let result = OutputGenerator::write_json_if_not_empty(json_values, output_path_str, "成功メッセージ");
        
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        let mut file = fs::File::open(output_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        assert!(contents.contains("\"customModes\""));
        assert!(contents.contains("\"name\": \"Test1\""));
        assert!(contents.contains("\"value\": 123"));
        assert!(contents.contains("\"name\": \"Test2\""));
        assert!(contents.contains("\"value\": 456"));
    }
    
    #[test]
    fn test_write_json_if_not_empty_with_empty_array() {
        let json_values: Vec<Value> = vec![];
        let result = OutputGenerator::write_json_if_not_empty(json_values, "nonexistent_path.json", "成功メッセージ");
        
        assert!(result.is_ok());
        assert!(!Path::new("nonexistent_path.json").exists());
    }
    
    #[test]
    fn test_write_content_if_not_empty_with_content() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_content.txt");
        let output_path_str = output_path.to_str().unwrap();
        
        let content = "Test\nMultiple lines".to_string();
        
        let result = OutputGenerator::write_content_if_not_empty(content, output_path_str, "成功メッセージ");
        
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        let file_content = fs::read_to_string(output_path).unwrap();
        assert_eq!(file_content, "Test\nMultiple lines");
    }
    
    #[test]
    fn test_write_content_if_not_empty_with_empty_string() {
        let content = "".to_string();
        let result = OutputGenerator::write_content_if_not_empty(content, "nonexistent_path.txt", "成功メッセージ");
        
        assert!(result.is_ok());
        assert!(!Path::new("nonexistent_path.txt").exists());
    }
}
