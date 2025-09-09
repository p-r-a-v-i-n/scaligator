use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the config file
    #[arg(short, long)]
    pub config: Option<String>,
}
