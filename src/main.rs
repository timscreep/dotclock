use clap::Parser;
mod cli;
mod clock;
mod config;

use crate::{cli::Args, clock::DotClock, config::ClockConfig};

fn main() {
    let args = Args::parse();
    let config =
        ClockConfig::load_or_create("dotclock", "config.toml").expect("Failed to load config");
    let merged_config = config.merge_args(&args);

    match merged_config.mode.as_str() {
        "tui" => println!("TUI not implemented"),
        "cli" | _ => {
            let clock = DotClock::new(&merged_config);
            if args.once {
                clock.display();
            } else {
                clock.run_loop();
            }
        }
    }
}
