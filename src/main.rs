use serde_json::{Value};
use clap::{Parser};
#[derive(Parser)]
struct Cli {
    json: String,
}
pub fn json_to_csv(json: &String) -> String {
    let v: Vec<Value> = serde_json::from_str(&json).unwrap();
    let mut header: String = String::new();
    let mut print_header = true;
    let mut csv = String::new();
    for item in &v {
        let mut row = String::new();
        for (key, value) in item.as_object().unwrap(){
            if print_header {
                header = header + key + ",";
            }
            row = row + &value.to_string() + ",";
        }
        if print_header {
            header.pop();
            csv = header.chars().collect::<String>();
            csv += "\n";
            print_header = false;
        }
        row.pop();
        row += "\n";
        csv = csv + &row;
    }
    csv
}

fn main() {
    let args = Cli::parse();
    let v: Vec<Value> = serde_json::from_str(&args.json).unwrap();
    let res = json_to_csv(&args.json);
    println!("{}", res)

}
//cargo run -- '[{"a": 4, "b": 8}, {"a":9, "b": 10}]'

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn two_dimension() {
    let parsed = json_to_csv(&String::from("[{\"a\": 4, \"b\": 8}, {\"a\":9, \"b\": 10}]"));
    assert_eq!(format!("{}", parsed), "a,b\n4,8\n9,10\n");
  }

  #[test]
  fn two_dimension_with_odds() {
    let parsed = json_to_csv(&String::from("[{\"a\": 4, \"b\": 8}, {\"a\":9}]"));
    assert_eq!(format!("{}", parsed), "a,b\n4,8\n9,\n");
  }
}