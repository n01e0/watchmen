use yara::*;
use std::path::Path;

pub struct Scanner {
    rule: Rules,
    path: Path,
    
}

pub fn scan(rules: &Rules, path: String, verbose: bool) {
    match rules.scan_file(path.clone(), 5) {
        Ok(v) => {
            for rule in v {
                eprintln!("Rules {} matched! {}", rule.identifier, path);
                if verbose {
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
            }
        }
        Err(e) => {
            if verbose {
                eprintln!("Error in {} -> {}", path, e);
            }
        }
    }
}
