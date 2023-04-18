mod coin;
mod display;
mod tracker;
mod compute;
mod args;

use clap::Parser;
use crate::args::Args;

fn main() {

    // Parse the arguments.
    let args = Args::parse();

    // Run and get results accoring to argument specifications.
    let results = compute::multi_thread(args.iterations, args.threads)
        .get_results();

    // Format and print results.
    display::display(&results);
}
