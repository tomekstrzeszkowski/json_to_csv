use clap::{arg, ArgAction, Parser};


#[derive(Parser)]
pub struct Cli {
    pub json: String,
    #[arg(short, long, action(ArgAction::SetTrue))]
    pub debug: bool,
}
