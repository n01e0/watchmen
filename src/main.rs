#[macro_use]
extern crate clap;

use std::fs;
use walkdir::WalkDir;
use yara::*;

mod scanner;

fn main() {
    let matches = clap_app!(watchmen =>
        (version:       crate_version!())
        (author:        crate_authors!())
        (about:         crate_description!())
        (@arg quiet: -q --quiet "quiet mode")
        (@arg verbose: -v --verbose "verbose mode")
        (@arg rule:     +required "yara rule file")
        (@arg target:   +required "Scan target file")
    )
    .get_matches();

    let mut compiler = Compiler::new().unwrap();

    compiler
        .add_rules_file(matches.value_of("rule").unwrap())
        .expect("Can't add rules");
    let rules = compiler.compile_rules().expect("Can't compile rules");

    let target = matches.value_of("target").unwrap();

    if fs::metadata(target).expect("Can't get metadata").is_dir() {
        for entry in WalkDir::new(target)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !fs::metadata(e.path()).expect("Can't get metadata").is_dir())
        {
            if !matches.is_present("quiet") {
                println!("scanning ... {}", entry.path().display());
            }
            scanner::scan(
                &rules,
                entry.path().to_str().unwrap().into(),
                matches.is_present("verbose"),
            );
        }
    } else {
        scanner::scan(&rules, target.into(), matches.is_present("verbose"));
    }
}
