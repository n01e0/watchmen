use yara::*;

pub fn scan(rules: &Rules, path: String) {
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
        }
        Err(e) => eprintln!("Error in {} -> {}", path, e),
    }
}
