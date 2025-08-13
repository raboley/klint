//! Configuration management module

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use crate::case_detector::NamingCase;

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub rules: RulesConfig,
    
    #[serde(default)]
    pub output: OutputConfig,
    
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Linting rules configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RulesConfig {
    #[serde(default = "default_table_naming")]
    pub table_naming: String,
    
    #[serde(default)]
    pub excluded_tables: Vec<String>,
}

/// Output configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub format: String,
    
    #[serde(default)]
    pub colors: bool,
    
    #[serde(default)]
    pub report_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rules: RulesConfig::default(),
            output: OutputConfig::default(),
            exclude: Vec::new(),
        }
    }
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            table_naming: default_table_naming(),
            excluded_tables: Vec::new(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: default_format(),
            colors: true,
            report_path: None,
        }
    }
}

fn default_table_naming() -> String {
    "PascalCase".to_string()
}

fn default_format() -> String {
    "terminal".to_string()
}

impl Config {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        debug!("Loading configuration from: {:?}", path);
        
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        
        let config: Config = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;
        
        info!("Configuration loaded from: {:?}", path);
        Ok(config)
    }

    /// Find configuration file in current or parent directories
    pub fn find_config() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        
        loop {
            let config_path = current.join(".klint.yml");
            if config_path.exists() {
                debug!("Found config file at: {:?}", config_path);
                return Some(config_path);
            }
            
            let config_path = current.join(".klint.yaml");
            if config_path.exists() {
                debug!("Found config file at: {:?}", config_path);
                return Some(config_path);
            }
            
            if !current.pop() {
                break;
            }
        }
        
        None
    }

    /// Get the naming case from configuration
    pub fn naming_case(&self) -> NamingCase {
        NamingCase::from_str(&self.rules.table_naming)
    }

    /// Check if a table is excluded
    pub fn is_table_excluded(&self, table_name: &str) -> bool {
        self.rules.excluded_tables.iter().any(|pattern| {
            // Simple pattern matching for now
            pattern == table_name || pattern == "*"
        })
    }

    /// Save configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        info!("Configuration saved to: {:?}", path);
        Ok(())
    }

    /// Create a default configuration file
    pub fn create_default() -> Self {
        Config::default()
    }
}

// Schema types for extended configuration (for future use)

/// Extended configuration schema with all options
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FullConfig {
    pub version: String,
    pub rules: ExtendedRules,
    pub output: ExtendedOutput,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
}

/// Extended rules with future options
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtendedRules {
    pub table_naming: TableNamingRule,
    pub excluded_tables: Vec<String>,
    // Future: column_naming, function_naming, etc.
}

/// Table naming rule configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableNamingRule {
    pub convention: String,
    pub severity: String, // error, warning, info
    pub auto_fix: bool,
}

/// Extended output configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtendedOutput {
    pub format: String,
    pub colors: bool,
    pub report_path: Option<String>,
    pub verbose: u8,
    pub quiet: bool,
}