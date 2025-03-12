use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug)]
pub struct ClinePaths {
    pub base: PathBuf,
    pub modes: PathBuf,
    pub rules: PathBuf,
}

impl ClinePaths {
    pub fn new() -> Self {
        let base = PathBuf::from(".cline");
        let modes = base.join("modes");
        let rules = base.join("rules");
        
        Self { base, modes, rules }
    }
    
    pub fn base_exists(&self) -> bool {
        self.base.exists()
    }
    
    pub fn modes_exists(&self) -> bool {
        self.modes.exists()
    }
    
    pub fn rules_exists(&self) -> bool {
        self.rules.exists()
    }
    
    pub fn get_global_config_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        
        // $HOME/.cline
        if let Some(home_dir) = home::home_dir() {
            let home_cline = home_dir.join(".cline");
            if home_cline.exists() && home_cline.is_dir() {
                paths.push(home_cline);
            }
            
            // $HOME/.config/cline
            let config_cline = home_dir.join(".config").join("cline");
            if config_cline.exists() && config_cline.is_dir() {
                paths.push(config_cline);
            }
        }
        
        paths
    }
    
    pub fn create_directories(&self) -> std::io::Result<()> {
        if !self.base.exists() {
            fs::create_dir_all(&self.base)?;
        }
        
        if !self.modes.exists() {
            fs::create_dir_all(&self.modes)?;
        }
        
        if !self.rules.exists() {
            fs::create_dir_all(&self.rules)?;
        }
        
        Ok(())
    }

    pub fn copy_dir_contents<P: AsRef<Path>, Q: AsRef<Path>>(
        src_dir: P,
        dest_dir: Q
    ) -> std::io::Result<usize> {
        let src_dir = src_dir.as_ref();
        let dest_dir = dest_dir.as_ref();
        
        if !src_dir.exists() || !src_dir.is_dir() {
            return Ok(0);
        }
        
        if !dest_dir.exists() {
            fs::create_dir_all(dest_dir)?;
        }
        
        let mut copied_count = 0;
        
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let src_path = entry.path();
            
            if src_path.is_file() {
                if let Some(file_name) = src_path.file_name() {
                    let dest_path = dest_dir.join(file_name);
                    fs::copy(&src_path, &dest_path)?;
                    copied_count += 1;
                }
            }
        }
        
        Ok(copied_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_creates_correct_paths() {
        let paths = ClinePaths::new();
        
        assert_eq!(paths.base, PathBuf::from(".cline"));
        assert_eq!(paths.modes, PathBuf::from(".cline").join("modes"));
        assert_eq!(paths.rules, PathBuf::from(".cline").join("rules"));
    }
    
    #[test]
    fn test_path_components() {
        let paths = ClinePaths::new();
        
        let expected_modes_path = paths.base.join("modes");
        assert_eq!(paths.modes, expected_modes_path);
        
        let expected_rules_path = paths.base.join("rules");
        assert_eq!(paths.rules, expected_rules_path);
    }
}
