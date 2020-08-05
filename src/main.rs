#[macro_use]
extern crate clap;

use yara::*;
use walkdir::WalkDir;
use std::fs;

fn main() {
    let matches = clap_app!(watchmen =>
            (version:       crate_version!())
            (author:        crate_authors!())
            (about:         crate_description!())
            (@arg rule:     +required "yara rule file")
            (@arg target:   +required "Scan target file")
        ).get_matches();

    let mut compiler = Compiler::new().unwrap();
    compiler.add_rules_file(matches.value_of("rule").unwrap()).expect("Can't add rules");
    let rules = compiler.compile_rules().expect("Can't compile rules");
    
    let target = matches.value_of("target").unwrap();
    
    if fs::metadata(target).expect("Can't get metadata").is_dir() {
        for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
            println!("scanning ... {}", entry.path().display());
            scan(&rules, entry.path().to_str().unwrap().into());
        }
    }

}

fn scan(rules: &Rules, path: String) {
    match rules.scan_file(path.clone(), 5) {
        Ok(v) => {
            for rule in v {
                eprintln!("Rules {} matched! {}", rule.identifier, path);
                println!("  matched strings:");
                for s in &rule.strings {
                    println!("    identifier:\t{}", s.identifier);
                    println!("    matches:");
                    for m in &s.matches {
                        println!("      offset:\t{}", m.offset);
                        println!("      length:\t{}", m.length);
                        println!("      %data:\t{:?}", m.data);
                    }
                }
            }
        },
        Err(e) => eprintln!("Error in {} -> {}", path, e),
    }
}
