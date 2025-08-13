//! Configuration management module

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use crate::case_detector::NamingCase;

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
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

/// Builder for creating Config instances with fluent API
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    rules: Option<RulesConfig>,
    output: Option<OutputConfig>,
    exclude: Option<Vec<String>>,
}

impl ConfigBuilder {
    /// Create a new ConfigBuilder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the rules configuration
    pub fn rules(mut self, rules: RulesConfig) -> Self {
        self.rules = Some(rules);
        self
    }
    
    /// Set the table naming convention
    pub fn table_naming<S: Into<String>>(mut self, naming: S) -> Self {
        let mut rules = self.rules.unwrap_or_default();
        rules.table_naming = naming.into();
        self.rules = Some(rules);
        self
    }
    
    /// Add an excluded table
    pub fn exclude_table<S: Into<String>>(mut self, table: S) -> Self {
        let mut rules = self.rules.unwrap_or_default();
        rules.excluded_tables.push(table.into());
        self.rules = Some(rules);
        self
    }
    
    /// Set multiple excluded tables
    pub fn exclude_tables<I, S>(mut self, tables: I) -> Self 
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut rules = self.rules.unwrap_or_default();
        rules.excluded_tables.extend(tables.into_iter().map(|t| t.into()));
        self.rules = Some(rules);
        self
    }
    
    /// Set the output configuration
    pub fn output(mut self, output: OutputConfig) -> Self {
        self.output = Some(output);
        self
    }
    
    /// Set the output format
    pub fn output_format<S: Into<String>>(mut self, format: S) -> Self {
        let mut output = self.output.unwrap_or_default();
        output.format = format.into();
        self.output = Some(output);
        self
    }
    
    /// Enable or disable colored output
    pub fn colors(mut self, colors: bool) -> Self {
        let mut output = self.output.unwrap_or_default();
        output.colors = colors;
        self.output = Some(output);
        self
    }
    
    /// Set the report output path
    pub fn report_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        let mut output = self.output.unwrap_or_default();
        output.report_path = Some(path.into());
        self.output = Some(output);
        self
    }
    
    /// Set file exclusion patterns
    pub fn exclude<I, S>(mut self, patterns: I) -> Self 
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude = Some(patterns.into_iter().map(|p| p.into()).collect());
        self
    }
    
    /// Add a single exclusion pattern
    pub fn exclude_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        let mut exclude = self.exclude.unwrap_or_default();
        exclude.push(pattern.into());
        self.exclude = Some(exclude);
        self
    }
    
    /// Build the final Config instance
    pub fn build(self) -> Config {
        Config {
            rules: self.rules.unwrap_or_default(),
            output: self.output.unwrap_or_default(),
            exclude: self.exclude.unwrap_or_default(),
        }
    }
    
    /// Build and validate the Config instance
    pub fn build_validated(self) -> Result<Config> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }
}

impl Config {
    /// Create a new ConfigBuilder for fluent configuration building
    /// 
    /// # Returns
    /// 
    /// A new ConfigBuilder instance for creating configurations with a fluent API
    /// 
    /// # Examples
    /// 
    /// ```
    /// use klint::Config;
    /// 
    /// let config = Config::builder()
    ///     .table_naming("snake_case")
    ///     .exclude_table("legacy_table")
    ///     .colors(false)
    ///     .build();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Load configuration from a YAML file
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the configuration file
    /// 
    /// # Returns
    /// 
    /// The loaded configuration, or an error if the file cannot be read or parsed
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use klint::Config;
    /// 
    /// let config = Config::from_file(".klint.yml").unwrap();
    /// ```
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

    /// Load and validate configuration from a YAML file
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the configuration file
    /// 
    /// # Returns
    /// 
    /// The loaded and validated configuration, or an error if the file cannot be
    /// read, parsed, or validated
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use klint::Config;
    /// 
    /// let config = Config::from_file_validated(".klint.yml").unwrap();
    /// ```
    pub fn from_file_validated<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = Self::from_file(path)?;
        config.validate()
            .with_context(|| "Configuration validation failed")?;
        Ok(config)
    }

    /// Find configuration file in current or parent directories
    /// 
    /// Searches for `.klint.yml` or `.klint.yaml` files starting from the current
    /// directory and walking up the directory tree.
    /// 
    /// # Returns
    /// 
    /// Some(PathBuf) if a configuration file is found, None otherwise
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
    /// 
    /// # Returns
    /// 
    /// The configured naming case for table names
    pub fn naming_case(&self) -> NamingCase {
        NamingCase::try_from(self.rules.table_naming.as_str()).unwrap_or(NamingCase::Unknown)
    }

    /// Check if a table is excluded from linting
    /// 
    /// # Arguments
    /// 
    /// * `table_name` - The name of the table to check
    /// 
    /// # Returns
    /// 
    /// True if the table should be excluded from linting
    pub fn is_table_excluded(&self, table_name: &str) -> bool {
        self.rules.excluded_tables.iter().any(|pattern| {
            // Support glob patterns
            if pattern == "*" {
                true
            } else if pattern.contains('*') || pattern.contains('?') {
                // Use glob pattern matching
                match glob::Pattern::new(pattern) {
                    Ok(glob_pattern) => glob_pattern.matches(table_name),
                    Err(_) => pattern == table_name, // Fallback to exact match
                }
            } else {
                pattern == table_name
            }
        })
    }

    /// Save configuration to a YAML file
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path where the configuration should be saved
    /// 
    /// # Returns
    /// 
    /// Ok(()) if successful, or an error if the file cannot be written
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        info!("Configuration saved to: {:?}", path);
        Ok(())
    }

    /// Create a default configuration
    /// 
    /// # Returns
    /// 
    /// A Config instance with default settings (PascalCase table naming)
    pub fn create_default() -> Self {
        Config::default()
    }

    /// Validate the configuration and return helpful error messages
    /// 
    /// # Returns
    /// 
    /// Ok(()) if the configuration is valid, or an error with detailed information
    /// about what is invalid and how to fix it
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        // Validate table naming convention
        if NamingCase::try_from(self.rules.table_naming.as_str()).is_err() {
            let valid_options = ["PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case"];
            errors.push(format!(
                "Invalid table naming convention '{}'. Valid options are: {}",
                self.rules.table_naming,
                valid_options.join(", ")
            ));
        }

        // Validate output format
        let valid_formats = ["terminal", "json"];
        if !valid_formats.contains(&self.output.format.as_str()) {
            errors.push(format!(
                "Invalid output format '{}'. Valid options are: {}",
                self.output.format,
                valid_formats.join(", ")
            ));
        }

        // Validate excluded tables (no empty strings)
        for (index, table) in self.rules.excluded_tables.iter().enumerate() {
            if table.trim().is_empty() {
                errors.push(format!(
                    "Empty table name in excluded_tables at position {}. Remove empty entries or use '*' for all tables",
                    index
                ));
            }
        }

        // Validate exclude patterns (no empty strings)
        for (index, pattern) in self.exclude.iter().enumerate() {
            if pattern.trim().is_empty() {
                errors.push(format!(
                    "Empty pattern in exclude at position {}. Remove empty entries",
                    index
                ));
            }
        }

        // Return all errors if any found
        if !errors.is_empty() {
            let error_message = format!(
                "Configuration validation failed:\n{}",
                errors.iter()
                    .enumerate()
                    .map(|(i, err)| format!("  {}: {}", i + 1, err))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            bail!(error_message);
        }

        Ok(())
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