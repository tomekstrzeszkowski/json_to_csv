use clap::Parser;
use serde_json::Value;
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
    let v: Value = serde_json::from_str(&args.json).unwrap();
    let mut res: String = String::from("");
    match v {
        Value::Object(_) => {
            res = json_to_csv::item_to_csv(v);
        },
        Value::Array(_) => {
            res = json_to_csv::items_to_csv(v);
        },
        _ => println!("JSON value is neither an object nor array")
    }
    println!("{}", res);
}
