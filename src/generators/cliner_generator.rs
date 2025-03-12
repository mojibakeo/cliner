use crate::error::{ClinerError, Result};
use crate::models::ClinePaths;
use crate::processors::FileProcessor;
use crate::generators::OutputGenerator;

pub struct ClinerGenerator {
    paths: ClinePaths,
}

impl ClinerGenerator {
    pub fn new() -> Self {
        Self {
            paths: ClinePaths::new(),
        }
    }
    
    pub fn validate_cline_exists(&self) -> Result<()> {
        if self.paths.base_exists() {
            return Ok(());
        }
        
        eprintln!("Error: .cline directory not found");
        Err(ClinerError::InvalidFormat(".cline directory not found".to_string()))
    }
    
    pub fn generate_roomodes(&self) -> Result<()> {
        if !self.paths.modes_exists() {
            println!("modes directory not found, skipping .roomodes generation");
            return Ok(());
        }
    
        let sorted_mode_entries = FileProcessor::collect_sorted_entries(&self.paths.modes)?;
        let modes_json_values = FileProcessor::convert_entries_to_json(sorted_mode_entries);
        OutputGenerator::write_json_if_not_empty(modes_json_values, ".roomodes", "Generated .roomodes")
    }
    
    pub fn generate_clinerules(&self) -> Result<()> {
        if !self.paths.rules_exists() {
            println!("rules directory not found, skipping .clinerules generation");
            return Ok(());
        }
    
        let sorted_rule_entries = FileProcessor::collect_sorted_entries(&self.paths.rules)?;
        let concatenated_rules = FileProcessor::concatenate_entries(sorted_rule_entries);
        OutputGenerator::write_content_if_not_empty(concatenated_rules, ".clinerules", "Generated .clinerules")
    }
    
    pub fn run_generate(&self) -> Result<()> {
        self.validate_cline_exists()?;
        
        let (roomodes_result, clinerules_result) = rayon::join(
            || self.generate_roomodes(),
            || self.generate_clinerules()
        );
        
        roomodes_result?;
        clinerules_result?;
        
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, create_dir_all, File};
    use std::io::Write;
    use tempfile::TempDir;
    use std::path::{Path, PathBuf};

    fn create_test_cline_directory(temp_dir: &TempDir) -> std::io::Result<(PathBuf, PathBuf)> {
        let cline_dir = temp_dir.path().join(".cline");
        let modes_dir = cline_dir.join("modes");
        let rules_dir = cline_dir.join("rules");
        
        create_dir_all(&modes_dir)?;
        create_dir_all(&rules_dir)?;

        let mode_content = "mode_name: TestMode\ngroups:\n- read\n- edit\n\n---\n\n# Test Role Definition\n\nThis is test.";
        let mode_file_path = modes_dir.join("test_mode.md");
        let mut mode_file = File::create(&mode_file_path)?;
        write!(mode_file, "{}", mode_content)?;

        let rule_content = "# Test Rule\n\nThis is rules for testing";
        let rule_file_path = rules_dir.join("test_rule.md");
        let mut rule_file = File::create(&rule_file_path)?;
        write!(rule_file, "{}", rule_content)?;

        Ok((modes_dir, rules_dir))
    }

    fn cleanup_generated_files(dir: &Path) -> std::io::Result<()> {
        let roomodes_path = dir.join(".roomodes");
        let clinerules_path = dir.join(".clinerules");
        
        if roomodes_path.exists() {
            fs::remove_file(&roomodes_path)?;
        }
        if clinerules_path.exists() {
            fs::remove_file(&clinerules_path)?;
        }
        Ok(())
    }

    struct TestClinerGenerator {
        paths: ClinePaths,
    }
    impl TestClinerGenerator {
        fn new(base_dir: &Path) -> Self {
            let base = base_dir.join(".cline");
            let modes = base.join("modes");
            let rules = base.join("rules");
            
            Self {
                paths: ClinePaths { base, modes, rules }
            }
        }
        
        fn validate_cline_exists(&self) -> Result<()> {
            if self.paths.base_exists() {
                return Ok(());
            }
            eprintln!("Error: .cline directory not found");
            Err(ClinerError::InvalidFormat(".cline directory not found".to_string()))
        }
        
        fn generate_roomodes_with_path(&self, output_path: &str) -> Result<()> {
            if !self.paths.modes_exists() {
                println!("modes directory not found, skipping .roomodes generation");
                return Ok(());
            }
            
            let sorted_mode_entries = FileProcessor::collect_sorted_entries(&self.paths.modes)?;
            let modes_json_values = FileProcessor::convert_entries_to_json(sorted_mode_entries);
            OutputGenerator::write_json_if_not_empty(modes_json_values, output_path, "Generated .roomodes")
        }
        
        fn generate_clinerules_with_path(&self, output_path: &str) -> Result<()> {
            if !self.paths.rules_exists() {
                println!("rules directory not found, skipping .clinerules generation");
                return Ok(());
            }
            
            let sorted_rule_entries = FileProcessor::collect_sorted_entries(&self.paths.rules)?;
            let concatenated_rules = FileProcessor::concatenate_entries(sorted_rule_entries);
            OutputGenerator::write_content_if_not_empty(concatenated_rules, output_path, "Generated .clinerules")
        }
        
        fn run_with_paths(&self, roomodes_path: &str, clinerules_path: &str) -> Result<()> {
            self.validate_cline_exists()?;
            
            let roomodes_result = self.generate_roomodes_with_path(roomodes_path);
            let clinerules_result = self.generate_clinerules_with_path(clinerules_path);
            
            roomodes_result?;
            clinerules_result?;
            
            Ok(())
        }
    }
    
    #[test]
    fn test_validate_cline_exists_when_exists() {
        let temp_dir = TempDir::new().unwrap();
        let (_modes_dir, _rules_dir) = create_test_cline_directory(&temp_dir).unwrap();
        
        let generator = TestClinerGenerator::new(temp_dir.path());
        let result = generator.validate_cline_exists();
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_roomodes() {
        let temp_dir = TempDir::new().unwrap();
        let (_modes_dir, _) = create_test_cline_directory(&temp_dir).unwrap();
        
        let generator = TestClinerGenerator::new(temp_dir.path());
        
        let roomodes_path = temp_dir.path().join(".roomodes").to_str().unwrap().to_string();
        let result = generator.generate_roomodes_with_path(&roomodes_path);
        
        assert!(result.is_ok());
        
        let roomodes_file = temp_dir.path().join(".roomodes");
        assert!(roomodes_file.exists());
        
        let content = fs::read_to_string(&roomodes_file).unwrap();
        assert!(content.contains("TestMode"));
        
        cleanup_generated_files(temp_dir.path()).unwrap();
    }

    #[test]
    fn test_generate_clinerules() {
        let temp_dir = TempDir::new().unwrap();
        let (_, _rules_dir) = create_test_cline_directory(&temp_dir).unwrap();
        
        let generator = TestClinerGenerator::new(temp_dir.path());
        
        let clinerules_path = temp_dir.path().join(".clinerules").to_str().unwrap().to_string();
        let result = generator.generate_clinerules_with_path(&clinerules_path);
        
        assert!(result.is_ok());
        
        let clinerules_file = temp_dir.path().join(".clinerules");
        assert!(clinerules_file.exists());
        
        let content = fs::read_to_string(&clinerules_file).unwrap();
        assert!(content.contains("Test Rule"));
        
        cleanup_generated_files(temp_dir.path()).unwrap();
    }

    #[test]
    fn test_run_generate() {
        let temp_dir = TempDir::new().unwrap();
        let (_modes_dir, _rules_dir) = create_test_cline_directory(&temp_dir).unwrap();
        
        let generator = TestClinerGenerator::new(temp_dir.path());
        
        let roomodes_path = temp_dir.path().join(".roomodes").to_str().unwrap().to_string();
        let clinerules_path = temp_dir.path().join(".clinerules").to_str().unwrap().to_string();
        
        let result = generator.run_with_paths(&roomodes_path, &clinerules_path);
        
        assert!(result.is_ok());
        
        let roomodes_file = temp_dir.path().join(".roomodes");
        let clinerules_file = temp_dir.path().join(".clinerules");
        
        assert!(roomodes_file.exists());
        assert!(clinerules_file.exists());
        
        cleanup_generated_files(temp_dir.path()).unwrap();
    }
}
