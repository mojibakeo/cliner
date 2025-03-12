use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::error::Result;
use crate::models::Mode;
use crate::processors::markdown_parser::{MarkdownParser, markdown_to_json};

pub struct FileProcessor;

impl FileProcessor {
    pub fn collect_sorted_entries(directory: &Path) -> Result<Vec<fs::DirEntry>> {
        let directory_iterator = fs::read_dir(directory)?;
        
        let mut directory_entries = Vec::new();
        for entry_result in directory_iterator {
            match entry_result {
                Ok(entry) => directory_entries.push(entry),
                Err(_) => continue,
            }
        }
        
        directory_entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        Ok(directory_entries)
    }
    
    pub fn read_file_content(path: &Path) -> Result<String> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(e.into()),
        }
    }
    
    #[allow(dead_code)]
    pub fn convert_entries_to_modes(entries: Vec<fs::DirEntry>) -> Vec<Mode> {
        let mut modes = Vec::new();
        
        for entry in entries {
            let file_path = entry.path();
            let file_content = match Self::read_file_content(&file_path) {
                Ok(text) => text,
                Err(_) => continue,
            };
            
            match MarkdownParser::parse_to_mode(&file_content) {
                Ok(mode) => modes.push(mode),
                Err(_) => eprintln!("Warning: Skipping invalid Markdown in {}", file_path.display()),
            }
        }
        
        modes
    }
    
    pub fn convert_entries_to_json(entries: Vec<fs::DirEntry>) -> Vec<Value> {
        let mut json_values = Vec::new();
        
        for entry in entries {
            let file_path = entry.path();
            let file_content = match Self::read_file_content(&file_path) {
                Ok(text) => text,
                Err(_) => continue,
            };
            
            match markdown_to_json(&file_content) {
                Ok(json_value) => json_values.push(json_value),
                Err(_) => eprintln!("Warning: Skipping invalid Markdown in {}", file_path.display()),
            }
        }
        
        json_values
    }
    
    pub fn concatenate_entries(entries: Vec<fs::DirEntry>) -> String {
        let mut concatenated_content = String::new();
        
        for entry in entries {
            let file_path = entry.path();
            let file_content = match Self::read_file_content(&file_path) {
                Ok(content) => content,
                Err(_) => continue,
            };
            
            concatenated_content.push_str(&file_content);
            concatenated_content.push_str("\n");
        }
        
        concatenated_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::{tempdir, TempDir};

    fn create_temp_file(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(filename);
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", content).unwrap();
        file_path
    }

    #[test]
    fn test_read_file_content() {
        let temp_dir = tempdir().unwrap();
        let file_path = create_temp_file(&temp_dir, "test.txt", "テスト内容");
        
        let content = FileProcessor::read_file_content(&file_path).unwrap();
        assert_eq!(content, "テスト内容");
    }

    #[test]
    fn test_read_file_nonexistent() {
        let nonexistent_path = PathBuf::from("nonexistent_file.txt");
        let result = FileProcessor::read_file_content(&nonexistent_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_concatenate_entries() {
        let temp_dir = tempdir().unwrap();
        create_temp_file(&temp_dir, "file1.txt", "内容1");
        create_temp_file(&temp_dir, "file2.txt", "内容2");
    }
}
