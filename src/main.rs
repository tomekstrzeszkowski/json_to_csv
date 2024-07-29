use clap::Parser;
use repair_json;
mod cli;
mod json_to_csv;

fn main() {
    let args = cli::Cli::parse();
    if args.debug {
        println!("[DEBUGGER]");
        for argument in std::env::args_os() {
            println!("{argument:?}");
        }
    }
    let mut json_builder = repair_json::Builder::new();
	let _ = json_builder.update(&args.json);
	let json = json_builder.completed_string().unwrap();
    let csv_builder = json_to_csv::Builder::new(json);
    let res = csv_builder.csv();
    println!("{}", res);
}
