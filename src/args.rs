use clap::Parser;

/// Simulator of the infamous Two-Coin Problem.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of simualations to run.
    #[arg(short, long, default_value_t = 1000)]
    pub iterations: u32,
    
    /// Number of threads to use.
    #[arg(short, long, default_value_t = 2)]
    pub threads: u8,
}
