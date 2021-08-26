use once_cell::sync::Lazy;
use regex::Regex;
use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DEFAULT_CAPACITY: &'static str = &"10M";

static RE_CAPACITY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\A([1-9]\d*)([kmg])?\z").unwrap());

#[derive(Debug)]
pub(super) struct Options {
    pub file: String,
    pub key: String,
    pub capacity: u64,
    pub numeric: bool,
    pub reverse: bool,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [OPTIONS] FILE", program);
    print!("{}", opts.usage(&brief));
}

pub(super) fn parse_opts() -> Options {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    let mut opts = getopts::Options::new();

    opts.optopt("k", "key", "JSON key to sort", "KEY");
    opts.optopt(
        "c",
        "capacity",
        &format!("chunk capacity (default: {})", DEFAULT_CAPACITY),
        "SIZE",
    );
    opts.optflag("n", "numeric-sort", "sort fields numerically");
    opts.optflag("r", "reverse", "sort in reverse order");
    opts.optflag("v", "version", "print version and exit");
    opts.optflag("h", "help", "print usage and exit");

    let matches = opts.parse(&args[1..]).unwrap();

    if args.len() == 1 || matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0)
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        process::exit(0)
    }

    let key = matches
        .opt_str("k")
        .unwrap_or_else(|| panic!("'--key' option required"));

    let s_cap = matches
        .opt_get_default("c", DEFAULT_CAPACITY.to_string())
        .unwrap_or_else(|e| panic!("invalid capacity size: {}", e));

    let cap = if let Some(m) = RE_CAPACITY.captures(&s_cap) {
        let n = m.get(1).unwrap().as_str().parse::<u64>().unwrap();

        let c = match m.get(2) {
            Some(u) => match u.as_str() {
                "k" | "K" => 1024_u64,
                "m" | "M" => 1024_u64.pow(2),
                "g" | "G" => 1024_u64.pow(3),
                _ => panic!("invalid capacity unit: {}", s_cap),
            },
            None => 1,
        };

        n * c
    } else {
        panic!("invalid capacity size: {}", s_cap)
    };

    let num = matches.opt_present("n");
    let rev = matches.opt_present("r");

    let file = match matches.free.len() {
        1 => matches.free[0].to_string(),
        _ => {
            print_usage(&program, opts);
            process::exit(1)
        }
    };

    Options {
        file: file,
        key: key,
        capacity: cap,
        numeric: num,
        reverse: rev,
    }
}
