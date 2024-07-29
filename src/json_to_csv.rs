use std::collections::HashMap;
use serde_json::Value;

pub struct Builder {
    json: String,
}
impl Builder {
    #[must_use]
    pub fn new(json: String) -> Self {
        Self{json}
    }

    pub fn csv(self) -> String {
        let v: Value = serde_json::from_str(&self.json).unwrap();
        let res: String;
        match v {
            Value::Object(_) => {
                res = item_to_csv(v);
            },
            Value::Array(_) => {
                res = items_to_csv(v);
            },
            _ => res = String::from("JSON value is neither an object nor array")
        }
        res
    }
}

fn collect_from_object(item: &Value) -> Vec<&String> {
    let mut headers: Vec<&String> = Vec::new();
    for (key, _) in item.as_object().unwrap() {
        if !headers.contains(&key) {
            headers.push(key);
        }
    }
    headers
}

fn extend_header<'a>(header_extend: &mut Vec<&'a String>, header: Vec<&'a String>) {
    if header_extend.len() >= header.len() {
        for h in header {
            if !header_extend.contains(&h) {
                header_extend.push(&h);
            }
        }
    } else {
        for &h in header.iter().rev() {
            header_extend.retain(|&x| x != h);
            header_extend.insert(0, h);
        }
    }
}

fn create_header(header_items: &Vec<&String>) -> String {
    let mut csv = String::new();
    let header_len = header_items.len() - 1;
    for (index, h) in header_items.iter().enumerate() {
        csv += h;
        if header_len != index {
            csv += ",";
        }
    }
    csv + "\n"
}

fn create_rows(v: &serde_json::Map<String, serde_json::Value>, header_items: &Vec<&String>) -> String {
    let mut header_to_value: HashMap<&String, String> = HashMap::new();
    for (key, value) in v {
        header_to_value.insert(&key, value.to_string());
    }
    for header in header_items {
        if !header_to_value.contains_key(header) {
            header_to_value.insert(header, String::from(""));
        }
    }
    let mut row = Vec::new();
    for header in header_items {
        row.push(header_to_value.get(header).unwrap().clone());
    }
    row.join(",") + "\n"
}

pub fn items_to_csv(v: Value) -> String {
    let mut csv = String::new();
    let mut header_items: Vec<&String> = Vec::new();
    let v: Vec<Value> = v.as_array().unwrap().clone();
    for item in &v {
        let h = collect_from_object(item);
        extend_header(&mut header_items, h);
    }
    csv += &create_header(&header_items);
    for item in &v {
        csv += &create_rows(&item.as_object().unwrap(), &header_items);
    }
    csv
}

pub fn item_to_csv(v: Value) -> String {
    let v = v.as_object().unwrap().clone();
    let mut csv = String::new();
    let mut header_items: Vec<&String> = Vec::new();
    extend_header(&mut header_items, v.keys().collect());
    csv += &create_header(&header_items);
    csv + &create_rows(&v, &header_items)
}


#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn two_dimension() {
    let value = serde_json::from_str(&String::from("[{\"a\": 4, \"b\": 8}, {\"a\":9, \"b\": 10}]")).unwrap();
    let parsed = items_to_csv(value);
    assert_eq!(format!("{}", parsed), "a,b\n4,8\n9,10\n");
  }

  #[test]
  fn two_dimension_with_odds() {
    let value = serde_json::from_str(&String::from("[{\"a\": 4, \"b\": 8}, {\"a\":9}]")).unwrap();
    let parsed = items_to_csv(value);
    assert_eq!(format!("{}", parsed), "a,b\n4,8\n9,\n");
  }
  #[test]
  fn simple_object() {
    let value = serde_json::from_str(&String::from("{\"a\": 4, \"b\": 8}")).unwrap();
    let parsed = item_to_csv(value);
    assert_eq!(format!("{}", parsed), "a,b\n4,8\n");
  }
  #[test]
  fn object_with_null() {
    let value = serde_json::from_str(&String::from("{\"a\": null, \"b\": 8}")).unwrap();
    let parsed = item_to_csv(value);
    assert_eq!(format!("{}", parsed), "a,b\nnull,8\n");
  }
}