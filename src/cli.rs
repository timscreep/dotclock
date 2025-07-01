use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_parser = ["cli", "tui"])]
    pub mode: Option<String>,

    #[arg(long)]
    pub once: bool,

    #[arg(long)]
    pub show_date: Option<bool>,

    #[arg(long)]
    pub show_time: Option<bool>,

    #[arg(long)]
    pub offset: Option<String>,
}
