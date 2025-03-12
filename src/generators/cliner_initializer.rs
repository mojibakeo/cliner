use crate::error::Result;
use crate::models::ClinePaths;

pub struct ClinerInitializer {
    paths: ClinePaths,
}

impl ClinerInitializer {
    pub fn new() -> Self {
        Self {
            paths: ClinePaths::new(),
        }
    }
    
    pub fn run_init(&self) -> Result<()> {
        println!("Initializing .cline directory...");
        
        self.paths.create_directories()?;
        println!("Created .cline directory structure");
        
        self.copy_from_global_configs()?;
        
        Ok(())
    }
    
    fn copy_from_global_configs(&self) -> Result<()> {
        let global_paths = ClinePaths::get_global_config_paths();
        if global_paths.is_empty() {
            println!("No global config directories found");
            return Ok(());
        }
        
        println!("Found {} global config directories", global_paths.len());
        
        let mut copied_modes = false;
        let mut copied_rules = false;
        
        for global_path in global_paths {
            println!("Checking global config at: {}", global_path.display());
            
            let global_modes_dir = global_path.join("modes");
            if global_modes_dir.exists() && global_modes_dir.is_dir() && !copied_modes {
                match ClinePaths::copy_dir_contents(&global_modes_dir, &self.paths.modes) {
                    Ok(count) => {
                        if count > 0 {
                            println!("Copied {} mode files from {}", count, global_modes_dir.display());
                            copied_modes = true;
                        }
                    },
                    Err(e) => eprintln!("Error copying mode files: {}", e),
                }
            }
            
            let global_rules_dir = global_path.join("rules");
            if global_rules_dir.exists() && global_rules_dir.is_dir() && !copied_rules {
                match ClinePaths::copy_dir_contents(&global_rules_dir, &self.paths.rules) {
                    Ok(count) => {
                        if count > 0 {
                            println!("Copied {} rule files from {}", count, global_rules_dir.display());
                            copied_rules = true;
                        }
                    },
                    Err(e) => eprintln!("Error copying rule files: {}", e),
                }
            }
            
            if copied_modes && copied_rules {
                break;
            }
        }
        
        if !copied_modes {
            println!("No mode files found in global config directories");
        }
        
        if !copied_rules {
            println!("No rule files found in global config directories");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::Path;

    struct TestClinerInitializer {
        paths: ClinePaths,
    }
    
    impl TestClinerInitializer {
        fn new(base_dir: &Path) -> Self {
            let base = base_dir.join(".cline");
            let modes = base.join("modes");
            let rules = base.join("rules");
            
            Self {
                paths: ClinePaths { base, modes, rules }
            }
        }
        
        fn run_init(&self) -> Result<()> {
            self.paths.create_directories()?;
            Ok(())
        }
    }
    
    #[test]
    fn test_run_init() {
        let temp_dir = TempDir::new().unwrap();
        
        let cline_dir = temp_dir.path().join(".cline");
        let modes_dir = cline_dir.join("modes");
        let rules_dir = cline_dir.join("rules");
        
        assert!(!cline_dir.exists());
        assert!(!modes_dir.exists());
        assert!(!rules_dir.exists());
        
        let initializer = TestClinerInitializer::new(temp_dir.path());
        let result = initializer.run_init();
        
        assert!(result.is_ok());
        
        assert!(cline_dir.exists());
        assert!(modes_dir.exists());
        assert!(rules_dir.exists());
    }
}
