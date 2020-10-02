#[macro_use]
use serde_derive::*;
use toml;

#[derive(Debug, Deserialize)]
pub struct Log {
    path: Option<String>,
    level: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pathes:     Vec<String>,
    exclude:    Option<Vec<String>>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    log:    Option<Log>,
    rule:   Option<Rule>,
    target: Target,
}

impl Config {
    pub fn new(path: Option<String>) -> Result<Self, toml::de::Error> {
        match path {
            Some(content) => {
                toml::from_str::<Config>(&content)
            },
            None => {
                Ok(Config {
                    log: None,
                    rule: None,
                    target: Target {
                        pathes: vec!["./".to_string()],
                        exclude: None
                    }
                })
            }
        }
    }

}
