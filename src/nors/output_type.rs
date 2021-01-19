use crate::nors::ResultsByCountType;
use std::str::FromStr;

#[derive(Debug)]
pub enum OutputType {
    PlainText,
    JSON,
}

impl FromStr for OutputType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plaintext" => Ok(Self::PlainText),
            "json" => Ok(Self::JSON),
            _ => Err(()),
        }
    }
}

impl Default for OutputType {
    fn default() -> Self {
        Self::PlainText
    }
}

impl OutputType {
    pub fn print(&self, results: &ResultsByCountType) {
        match self {
            Self::PlainText => {
                for (result_type, result) in results.results.iter() {
                    println!("{:?}: {}", result_type, result);
                }
            }
            Self::JSON => {
                println!(
                    "{}",
                    serde_json::to_string(&results).expect("json serialize error.")
                );
            }
        }
    }
}
