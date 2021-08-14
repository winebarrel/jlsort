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
    let num = opts.numeric;
    let rev = opts.reverse;

    ex_merge_sort::sort_by(f, io::stdout(), opts.capacity, |a, b| {
        let mut a = json_value_from_str(a, &key);
        let mut b = json_value_from_str(b, &key);

        if rev {
            let tmp = a;
            a = b;
            b = tmp;
        }

        if num {
            let a = a.as_f64().unwrap_or(f64::INFINITY);
            let b = b.as_f64().unwrap_or(f64::INFINITY);
            a.partial_cmp(&b).unwrap()
        } else {
            let a = a.to_string();
            let a = a.trim_end_matches(|c| c == '\r' || c == '\n');
            let b = b.to_string();
            let b = b.trim_end_matches(|c| c == '\r' || c == '\n');
            a.partial_cmp(b).unwrap()
        }
    })
    .unwrap();
}
