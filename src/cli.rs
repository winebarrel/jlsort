use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DEFAULT_CAPACITY: u64 = 10485760;

#[derive(Debug)]
pub(super) struct Options {
    pub file: String,
    pub key: String,
    pub capacity: u64,
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

    let cap = matches
        .opt_get_default("c", DEFAULT_CAPACITY)
        .unwrap_or_else(|e| panic!("invalid capacity size: {}", e));

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
    }
}
