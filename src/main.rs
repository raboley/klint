use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "klint")]
#[command(version, about = "A linter for KQL files to enforce table naming conventions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to lint (file or directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Automatically fix violations
    #[arg(short, long)]
    fix: bool,

    /// Preview fixes without applying them (dry-run)
    #[arg(short, long)]
    preview: bool,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "terminal")]
    output: OutputFormat,

    /// Verbosity level (can be repeated)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Quiet mode (only show errors)
    #[arg(short, long)]
    quiet: bool,

    /// Generate report file
    #[arg(short, long, value_name = "FILE")]
    report: Option<PathBuf>,

    /// Recursive directory processing
    #[arg(short = 'R', long)]
    recursive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new configuration file
    Init {
        /// Force overwrite existing config
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Terminal,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let log_level = match (cli.quiet, cli.verbose) {
        (true, _) => "error",
        (false, 0) => "warn",
        (false, 1) => "info",
        (false, 2) => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(log_level))
        )
        .init();

    let exit_code = match cli.command {
        Some(Commands::Init { force }) => init_config(force)?,
        None => run_linter(&cli)?,
    };

    process::exit(exit_code);
}

fn init_config(force: bool) -> Result<i32> {
    use klint::config_template::default_config_yaml;
    use std::fs;
    use std::path::Path;
    
    let config_path = Path::new(".klint.yml");
    
    // Check if config already exists
    if config_path.exists() && !force {
        eprintln!("Configuration file already exists. Use --force to overwrite.");
        return Ok(1);
    }
    
    // Write default configuration template
    fs::write(config_path, default_config_yaml())?;
    
    println!("Configuration file created at .klint.yml");
    Ok(0)
}

fn run_linter(cli: &Cli) -> Result<i32> {
    use klint::{Config, Linter};
    use klint::file_processor::process_path;
    use klint::output::create_formatter;
    
    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        Config::from_file_validated(config_path)?
    } else if let Some(found_config) = Config::find_config() {
        Config::from_file_validated(found_config)?
    } else {
        Config::default()
    };
    
    // Create linter
    let linter = Linter::new(config);
    
    // Process files
    let files = process_path(&cli.path, cli.recursive)?;
    
    if files.is_empty() {
        println!("No KQL files found");
        return Ok(0);
    }
    
    let mut all_results = Vec::new();
    let mut has_violations = false;
    let mut total_fixes = 0;
    let mut all_previews = Vec::new();
    
    // Lint each file
    for file in &files {
        let content = klint::file_processor::read_file(file)?;
        let mut result = linter.lint_content(&content, Some(file))?;
        
        if result.has_violations() {
            has_violations = true;
            
            // Generate fix previews if --preview flag is enabled
            if cli.preview {
                use klint::Fixer;
                let previews = Fixer::preview_fixes(&result);
                all_previews.extend(previews);
            }
            
            // Apply fixes if --fix flag is enabled
            if cli.fix {
                use klint::Fixer;
                
                let fixed_content = Fixer::fix(&result)?;
                
                // Write the fixed content back to the file
                klint::file_processor::write_file(file, &fixed_content)?;
                
                total_fixes += result.violations.len();
                
                // Update result to reflect that violations were fixed
                result.violations.clear();
            }
        }
        
        all_results.push(result);
    }
    
    // Output results
    let output_format = match cli.output {
        OutputFormat::Terminal => klint::output::OutputFormat::Terminal,
        OutputFormat::Json => klint::output::OutputFormat::Json,
    };
    
    let formatter = create_formatter(output_format);
    let output = formatter.format_results(&all_results)?;
    print!("{}", output);
    
    let summary = formatter.format_summary(&all_results)?;
    println!("{}", summary);
    
    // Show fix information if fixes were applied
    if cli.fix && total_fixes > 0 {
        println!("Fixed {} violations", total_fixes);
    }
    
    // Show preview information if preview mode is enabled
    if cli.preview {
        match cli.output {
            OutputFormat::Terminal => {
                if all_previews.is_empty() {
                    println!("No fixes needed");
                } else {
                    println!("\nProposed fixes:");
                    for preview in &all_previews {
                        println!("  Line {}: {} → {}", preview.line, preview.original, preview.fixed);
                    }
                }
            }
            OutputFormat::Json => {
                // For JSON output, include previews in the JSON structure
                if !all_previews.is_empty() {
                    use serde_json::json;
                    let preview_json = json!({
                        "preview": all_previews.iter().map(|p| json!({
                            "original": p.original,
                            "fixed": p.fixed,
                            "line": p.line
                        })).collect::<Vec<_>>()
                    });
                    println!("{}", serde_json::to_string_pretty(&preview_json)?);
                }
            }
        }
    }
    
    // Return appropriate exit code
    if has_violations && !cli.fix {
        Ok(1) // Found violations (not fixed)
    } else {
        Ok(0) // No violations or violations were fixed
    }
}