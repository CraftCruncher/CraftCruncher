use std::env;

use getopts::Options;

fn main() {
    let mut opts = Options::new();
    opts.reqopt(
        "p",
        "plugin-portal",
        "Where to fetch the data from -p [plugin portal]",
        "spiget dbo hangar modrinth",
    );

    let args: Vec<String> = env::args().collect();
    let parsed = match opts.parse(&args) {
        Ok(parsed) => parsed,
        Err(e) => panic!("{}", e.to_string()),
    };
    let fetcher = parsed.opt_str("p").unwrap();
}
