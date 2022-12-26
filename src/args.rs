use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// Set file to read maximum memory from. If not set, we try files, configured
    /// in `max_memory_files`.
    #[arg(short='m', long, env("PREOOM_MAX_MEM_FILE"))]
    max_memory_file: Option<String>,

    /// Default files to for `max_memory_file` to try. First match will be taken.
    #[arg(short='M', long, action = clap::ArgAction::Append, default_values=&[
    "one",
    "two"
    ])]
    max_memory_files: Vec<String>,

    /// Set file to read maximum memory from. If not set, we try files, configured
    //  in `used_memory_files`.
    #[arg(short='u', long, env("PREOOM_USED_MEM_FILE"))]
    used_memory_file: Option<String>,

    /// Default files to for `used_memory_file` to try. First match will be taken.
    #[arg(short='U', long, action = clap::ArgAction::Append, default_values=&[
    "one",
    "two"
    ])]
    used_memory_files: Vec<String>,

    /// How often to check memory usage
    #[arg(short, long, default_value_t=1, env("PREOOM_INTERVAL_SEC"))]
    interval: u8,

    /// Maximum memory usage percent
    #[arg(short='p', long, default_value_t=90, env("PREOOM_MAX_MEM_PERC"))]
    // TODO andi: why this does not work?
    // #[clap(validator = _validate_percent)]
    max_memory_percent: u8,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn _validate_percent(max_usage_percent: u8) -> Result<(), String> {
    if max_usage_percent >= 100 {
        Err(String::from(
            "Using >= 100 percent of memory will never happen since the process would already be OOM",
        ))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    max_memory_file: String,
    used_memory_file: String,
    interval: u8,
    max_memory_percent: u8,
    verbose: u8,
}

impl Config {
    pub fn configure() -> Result<Config, String> {
        let args = Args::parse();

        // TODO andi: perform potential post-parsing actions here.
        Ok(Config{
            max_memory_file: args.max_memory_file.unwrap_or("one".to_string()),
            used_memory_file: args.used_memory_file.unwrap_or("two".to_string()),
            interval: args.interval,
            max_memory_percent: args.max_memory_percent,
            verbose: args.verbose,
        })
    }
}
