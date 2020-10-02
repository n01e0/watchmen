#[macro_use]
extern crate clap;

use std::process::exit;
use std::fs;
use walkdir::WalkDir;
use yara::*;

mod config;
mod scanner;

fn main() {
    let matches = clap_app!(watchmen =>
        (version:       crate_version!())
        (author:        crate_authors!())
        (about:         crate_description!())
        (@arg quiet: -q --quiet "quiet mode")
        (@arg verbose: -v --verbose "verbose mode")
        (@arg config: -c --config +takes_value "config file")
        (@arg daemonize: -d --daemonize "to daemonize")
        (@arg rule:     "yara rule file(or dir)")
        (@arg target:   "Scan target file(or dir)")
    )
    .get_matches();

    let mut config = match matches.value_of("config") {
        Some(path) => {
            let content = fs::read_to_string(path).unwrap_or_else(|e| {eprintln!("{}", e); exit(1); });
            match config::Config::new(Some(content)) {
                Ok(config) => {
                    println!("{:#?}", config);
                    config
                },
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        },
        None => {
            config::Config::new(None).unwrap()

            // Scannerに読み込む時に設定されてない部分をdefault valueで埋めようかな
        },
    };


}
