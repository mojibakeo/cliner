use std::path::PathBuf;

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
        
        // モードディレクトリのパスが正しく結合されているか検証
        let expected_modes_path = paths.base.join("modes");
        assert_eq!(paths.modes, expected_modes_path);
        
        // ルールディレクトリのパスが正しく結合されているか検証
        let expected_rules_path = paths.base.join("rules");
        assert_eq!(paths.rules, expected_rules_path);
    }
}
