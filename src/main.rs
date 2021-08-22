mod cli;
use serde_json::json;
use serde_json::Value;

use std::fs;
use std::io;

fn json_value_from_str(line: &str, key: &str) -> Value {
    let empty_str = json!("");

    let jv: Value = serde_json::from_str(line).unwrap();
    match jv.get(&key) {
        Some(v) => v.clone(),
        None => {
            eprintln!("warning: key '{}' not in '{}'", key, line.trim());
            empty_str
        }
    }
}

fn main() {
    let opts = cli::parse_opts();
    let f = fs::File::open(opts.file).unwrap();
    let key = opts.key;

    ex_merge_sort_by_key::sort_by_key(f, io::stdout(), opts.capacity, |l| {
        json_value_from_str(l, &key).to_string()
    })
    .unwrap();
}
