mod cli;
use serde_json::json;
use serde_json::Value;

use io::prelude::BufRead;
use std::fs;
use std::io;
use std::io::Seek;
use std::io::Write;

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

    if opts.file == "-" {
        let f = tempfile::tempfile().unwrap();
        copy(io::stdin(), &f).unwrap();
        sort(f, &opts.key, opts.numeric, opts.capacity, opts.reverse)
    } else {
        let f = fs::File::open(opts.file).unwrap();
        sort(f, &opts.key, opts.numeric, opts.capacity, opts.reverse)
    }
    .unwrap();
}

fn copy<I, O>(fin: I, fout: O) -> io::Result<()>
where
    I: io::Read,
    O: io::Write + io::Seek,
{
    let mut reader = io::BufReader::new(fin);
    let mut writer = io::BufWriter::new(fout);
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        writer.write(buf.as_bytes())?;
        buf.clear();
    }

    writer.seek(std::io::SeekFrom::Start(0))?;

    Ok(())
}

fn sort(f: fs::File, key: &str, numeric: bool, capacity: u64, reverse: bool) -> io::Result<()> {
    if numeric {
        sort_by_num_key(f, key, capacity, reverse)
    } else {
        sort_by_str_key(f, key, capacity, reverse)
    }
}

fn sort_by_str_key(f: fs::File, key: &str, capacity: u64, reverse: bool) -> io::Result<()> {
    ex_merge_sort_by_key::sort_by_key_with_order(f, io::stdout(), capacity, reverse, |l| {
        json_value_from_str(l, key).to_string()
    })
}

fn sort_by_num_key(f: fs::File, key: &str, capacity: u64, reverse: bool) -> io::Result<()> {
    ex_merge_sort_by_key::sort_by_key_with_order(f, io::stdout(), capacity, reverse, |l| {
        json_value_from_str(l, key).as_i64().unwrap_or(i64::MAX)
    })
}
